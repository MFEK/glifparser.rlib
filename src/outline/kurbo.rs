/// Kurbo module — warning: only guaranteed to round trip closed contours!

use float_cmp::{ApproxEq, F32Margin};
use itertools::Itertools as _;
use kurbo::{BezPath, PathEl, Shape as _};

use std::collections::VecDeque;

use super::{Contour, Outline};
use crate::error::GlifParserError;
use crate::outline::GlifOutline;
use crate::point::{GlifPoint, PointData, PointType, WhichHandle};

use super::RefigurePointTypes as _;
use crate::outline::contour::{PrevNext as _, State as _};

use PathEl::*;
use PointType::*;

pub trait FromKurbo: Sized {
    type Output;
    fn from_kurbo(kpath: &BezPath) -> Self::Output;
}

impl GlifPoint {
    fn from_kurbo(kp: kurbo::Point, pt: PointType) -> Self {
        Self::from_x_y_type((kp.x as f32, kp.y as f32), pt)
    }

    fn from_kurbo_offcurve(kp: kurbo::Point) -> Self {
        Self::from_kurbo(kp, PointType::OffCurve)
    }
}

impl From<PathEl> for PointType {
    fn from(el: PathEl) -> Self {
        match el {
            MoveTo(..) => Move,
            LineTo(..) => Line,
            QuadTo(..) => QCurve,
            CurveTo(..) => Curve,
            ClosePath => QClose,
        }
    }
}
impl From<&PathEl> for PointType {
    fn from(el: &PathEl) -> Self {
        (*el).into()
    }
}
impl From<&mut PathEl> for PointType {
    fn from(el: &mut PathEl) -> Self {
        (*el).into()
    }
}

pub trait IntoKurbo: Sized {
    fn into_kurbo(self) -> Result<BezPath, GlifParserError> {
        Ok(BezPath::from_vec(self.into_kurbo_vec()?))
    }
    fn into_kurbo_vec(self) -> Result<Vec<PathEl>, GlifParserError>;
}

impl<PD: PointData> IntoKurbo for Outline<PD> {
    fn into_kurbo_vec(self) -> Result<Vec<PathEl>, GlifParserError> {
        let ret = self
            .into_iter()
            .map(|c| c.into_kurbo_vec())
            .filter(|kv| kv.is_ok())
            .map(Result::unwrap)
            .flatten()
            .collect();

        Ok(ret)
    }
}

impl<PD: PointData> IntoKurbo for Contour<PD> {
    fn into_kurbo_vec(mut self) -> Result<Vec<PathEl>, GlifParserError> {
        let is_closed = self.is_closed();
        self.refigure_point_types();
        let mut kurbo_vec = vec![];

        if is_closed {
            kurbo_vec.push(PathEl::MoveTo(self.first().unwrap().as_kpoint()));
        }

        for (pi, point) in self.iter().enumerate() {
            kurbo_vec.push(match point.ptype {
                PointType::Move => PathEl::MoveTo(point.as_kpoint()),
                PointType::Line => PathEl::LineTo(point.as_kpoint()),
                PointType::QCurve => {
                    PathEl::QuadTo(point.handle_as_point(WhichHandle::A), point.as_kpoint())
                }
                PointType::Curve => match self.contour_prev_next(pi)? {
                    (_, Some(next)) => PathEl::CurveTo(
                        point.handle_as_point(WhichHandle::A),
                        self[next].handle_as_point(WhichHandle::B),
                        self[next].as_kpoint(),
                    ),
                    (Some(prev), None) => PathEl::CurveTo(
                        self[prev].handle_as_point(WhichHandle::A),
                        point.handle_as_point(WhichHandle::B),
                        point.as_kpoint(),
                    ),
                    (None, None) => unreachable!(),
                },
                ptype => return Err(GlifParserError::GlifContourHasBadPointType { pi, ptype }),
            });
        }

        if is_closed {
            if self.last().unwrap().ptype == Curve && self.first().unwrap().ptype == Curve {
                let lp = kurbo_vec.last().unwrap().clone();
                if let PathEl::CurveTo(p1, p2, _p3) = lp {
                    *kurbo_vec.last_mut().unwrap() = PathEl::CurveTo(
                        p1, p2, self.first().unwrap().as_kpoint()
                    );
                }
            }
            kurbo_vec.push(PathEl::ClosePath);
        }

        Ok(kurbo_vec)
    }
}

impl<PD: PointData> FromKurbo for Outline<PD> {
    type Output = Outline<PD>;
    fn from_kurbo(kpath: &BezPath) -> Outline<PD> {
        let mut on_points = VecDeque::new();
        let mut a_handles = VecDeque::new(); // "next"
        let mut b_handles = VecDeque::new(); // "prev"
        let mut breakers = VecDeque::new();
        let mut ptypes = VecDeque::new();

        for (kpi, el) in kpath.path_elements(1.0).enumerate() {
            let (a, b, on) = match el {
                MoveTo(kp) => {
                    (None, None, Some(kp))
                }
                LineTo(kp) => {
                    (None, None, Some(kp))
                }
                QuadTo(kpb, kp) => {
                    (None, Some(kpb), Some(kp))
                }
                CurveTo(kpa, kpb, kp) => {
                    (Some(kpa), Some(kpb), Some(kp))
                }
                ClosePath => {
                    (None, None, None)
                }
            };

            match el {
                ClosePath | MoveTo(..) => {
                    breakers.push_back(Some(PointType::from(el)));
                }
                _ => {
                    breakers.push_back(None);
                }
            }

            on_points.push_back((kpi, PointType::from(el), on));
            a_handles.push_back((kpi, PointType::from(el), a));
            b_handles.push_back((kpi, PointType::from(el), b));
            ptypes.push_back(PointType::from(el));
        }
        let mut on_points_no_double_close = VecDeque::new();
        let mut bad_kpi = VecDeque::new();
        for (k, mut g) in &(on_points.clone().into_iter().group_by(|(_, el, _)| *el == QClose)) {
            if !k {
                on_points_no_double_close.extend(g.collect::<VecDeque<_>>());
            } else {
                log::warn!("Kurbo bug — consecutive ClosePath's detected. Patching Kurbo vec.");
                on_points_no_double_close.push_back(g.next().unwrap());
                for (remainder_kpi, _, _) in g {
                    bad_kpi.push_back(remainder_kpi);
                }
            }
        }
        on_points = on_points_no_double_close;
        let mut next_handles: VecDeque<_> = a_handles.into_iter().filter(|(kpi, _, a)|a.is_some() && !bad_kpi.contains(kpi)).collect();
        let mut prev_handles: VecDeque<_> = b_handles.into_iter().filter(|(kpi, _, b)|b.is_some() && !bad_kpi.contains(kpi)).collect();
        //next_handles.rotate_right(1);
        //prev_handles.rotate_left(1);
        //next_handles.rotate_left(1);
        let mut outline = vec![];
        let mut contour = vec![];
        let mut open_closed = vec![];

        for (kpi, pt, kp) in on_points {
            match (kp, pt, contour.len()) {
                (Some(kpo), PointType::Move, 1..) => {
                    outline.push(contour);
                    contour = vec![(kpi, GlifPoint::from_kurbo(kpo, pt)); 1];
                    open_closed.push(true);
                },
                (Some(kpo), pt, _) => contour.push((kpi, GlifPoint::from_kurbo(kpo, pt))),
                (None, _, 0..=1) => {
                    log::warn!("Ignoring consecutive ClosePath / lone point — Kurbo bug?");
                },
                (None, _, 2..) => { // got a close path
                    let (_, lp) = contour.last().unwrap().clone();
                    if lp.ptype == PointType::Curve {
                        let (first_kpi, _kp) = contour.remove(0);
                        if let Some((first_prev, _)) = prev_handles.iter().find_position(|(pkpi, _, _)|*pkpi == first_kpi + 1) {
                            let ph = prev_handles.remove(first_prev).unwrap();
                            let nh = next_handles.remove(first_prev).unwrap();
                            let (cur_kpi, _) = prev_handles.iter().find_position(|(pkpi, _, _)|*pkpi == kpi - 1).unwrap();
                            next_handles.insert(cur_kpi + 1, nh);
                            prev_handles.insert(cur_kpi + 1, ph);
                        }
                    } else {
                        let (_, fp) = contour.first_mut().unwrap();
                        if !( fp.x.approx_eq(lp.x, F32Margin::default()) && fp.y.approx_eq(lp.y, F32Margin::default()) ) {
                            fp.ptype = PointType::Line;
                        }
                    }
                    outline.push(contour);
                    contour = vec![];
                    open_closed.push(false);
                },
                _ => unreachable!()
            }
        }

        let mut goutline = vec![];
        let mut gcontour = vec![];

        for c in outline {
            for (_, p) in c {
                gcontour.push(p.clone());
                let (a, b) = match p.ptype {
                    PointType::QCurve => {
                        (Some(next_handles.pop_front()), None)
                    },
                    PointType::Curve => {
                        (Some(next_handles.pop_front()), Some(prev_handles.pop_front()))
                    },
                    _ => continue
                };

                if let Some(Some((_, _, Some(akp)))) = a {
                    gcontour.push(GlifPoint::from_kurbo_offcurve(akp));
                }
                if let Some(Some((_, _, Some(bkp)))) = b {
                    gcontour.push(GlifPoint::from_kurbo_offcurve(bkp));
                }
            }
            goutline.push(gcontour);
            gcontour = vec![];
        }
        let goutline_t = GlifOutline::from(goutline);
        goutline_t.try_into().unwrap()
    }
}
