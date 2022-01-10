use kurbo::{BezPath, PathEl, Shape as _};

use std::collections::VecDeque;

use crate::error::GlifParserError;
use crate::point::{Handle, Point, PointType, PointData, WhichHandle};
use super::{Contour, Outline};

use crate::outline::contour::{PrevNext as _, State as _};
use super::RefigurePointTypes as _;

pub trait FromKurbo<PD: PointData>: Sized {
    fn from_kurbo(kpath: &BezPath) -> Outline<PD>;
}

pub trait IntoKurbo: Sized {
    fn into_kurbo(self) -> Result<BezPath, GlifParserError> {
        Ok(BezPath::from_vec(self.into_kurbo_vec()?))
    }
    fn into_kurbo_vec(self) -> Result<Vec<PathEl>, GlifParserError>;
}

impl<PD: PointData> IntoKurbo for Outline<PD> {
    fn into_kurbo_vec(self) -> Result<Vec<PathEl>, GlifParserError> {
        Ok(self.into_iter().map(|c|c.into_kurbo_vec()).filter(|kv|kv.is_ok()).map(Result::unwrap).flatten().collect())
    }
}

impl<PD: PointData> IntoKurbo for Contour<PD> {
    fn into_kurbo_vec(mut self) -> Result<Vec<PathEl>, GlifParserError> {
        let is_closed = self.is_closed();
        self.refigure_point_types();
        let mut kurbo_vec = vec![];

        for (pi, point) in self.iter().enumerate() {
            kurbo_vec.push(match point.ptype {
                PointType::Move => PathEl::MoveTo(point.as_kpoint()),
                PointType::Line => PathEl::LineTo(point.as_kpoint()),
                PointType::QCurve => PathEl::QuadTo(point.handle_as_point(WhichHandle::A), point.as_kpoint()),
                PointType::Curve => {
                    match self.contour_prev_next(pi)? {
                        (_, Some(next)) => PathEl::CurveTo(point.handle_as_point(WhichHandle::A), self[next].handle_as_point(WhichHandle::B), point.as_kpoint()),
                        (Some(prev), None) => PathEl::CurveTo(self[prev].handle_as_point(WhichHandle::A), point.handle_as_point(WhichHandle::B), point.as_kpoint()),
                        (None, None) => unreachable!()
                    }
                }
                ptype => return Err(GlifParserError::GlifContourHasBadPointType{pi, ptype})
            });
        }

        if is_closed {
            kurbo_vec.push(PathEl::ClosePath);
        }

        Ok(kurbo_vec)
    }
}

impl<PD: PointData> FromKurbo<PD> for Outline<PD> {
    fn from_kurbo(kpath: &BezPath) -> Outline<PD> {
        let ptvec: Vec<_> = kpath.path_elements(f64::NAN).map(|el| { // accuracy not used for BezPath's
            match el {
                PathEl::MoveTo(kp) => (PointType::Move, kp.x as f32, kp.y as f32, Handle::Colocated, Handle::Colocated),
                PathEl::LineTo(kp) => (PointType::Line, kp.x as f32, kp.y as f32, Handle::Colocated, Handle::Colocated),
                PathEl::QuadTo(kpa, kp) => (PointType::QCurve, kp.x as f32, kp.y as f32, Handle::At(kpa.x as f32, kpa.y as f32), Handle::Colocated),
                PathEl::CurveTo(kpa, kpb, kp) => (PointType::Curve, kp.x as f32, kp.y as f32, Handle::At(kpa.x as f32, kpa.y as f32), Handle::At(kpb.x as f32, kpb.y as f32)),
                PathEl::ClosePath => (PointType::Undefined, f32::NAN, f32::NAN, Handle::Colocated, Handle::Colocated)
            }
        }).chain(vec![(PointType::Undefined, 0., 0., Handle::Colocated, Handle::Colocated)]).collect();

        let mut ret = vec![];
        let mut now_vec: Vec<Point<PD>> = vec![];
        let mut open_closed = vec![];
        let mut a_handles = VecDeque::with_capacity(ptvec.len());
        let mut b_handles = VecDeque::with_capacity(ptvec.len());

        for (ptype, x, y, a, b) in ptvec {
            if ptype != PointType::Undefined {
                a_handles.push_back(a);
            }
            if ptype != PointType::Undefined {
                b_handles.push_back(b);
            }
            match (ptype, now_vec.is_empty()) {
                (PointType::Undefined, false) | (PointType::Move, false) => {
                    if ptype == PointType::Undefined && now_vec[0].ptype == PointType::Move {
                        let last_a = a_handles.pop_front().unwrap_or(Handle::Colocated);
                        let last_b = b_handles.pop_back().unwrap_or(Handle::Colocated);
                        let first = now_vec.remove(0);
                        let first_a = first.a;
                        let first_b = first.b;
                        now_vec.last_mut().map(|lp|lp.a = first_a);
                        now_vec.last_mut().map(|lp|lp.b = last_b);
                    }
                    let closed = now_vec[0].ptype != PointType::Move;
                    let now_len = now_vec.len();
                    for (idx, point) in now_vec.iter_mut().enumerate() {
                        if idx != 0 || closed {
                            point.ptype = PointType::Curve;
                        }
                        if idx == 0 && closed {
                            point.a = a_handles.pop_back().unwrap_or(Handle::Colocated);
                            point.b = b_handles.pop_back().unwrap_or(Handle::Colocated);
                        } else {
                            point.a = a_handles.pop_front().unwrap_or(Handle::Colocated);
                            point.b = b_handles.pop_front().unwrap_or(Handle::Colocated);
                        }
                    }
                    if !closed {
                        now_vec[now_len - 2].a = now_vec[now_len - 1].a;
                    }
                    #[cfg(debug_assertions)]
                    if !b_handles.is_empty() {
                        log::error!("B handles vec contained {} handles! {:?}", b_handles.len(), &b_handles);
                    }
                    b_handles.clear();
                    ret.push(now_vec);
                    now_vec = vec![];
                    open_closed.push(closed);
                }
                _ => ()
            }
            if ptype != PointType::Undefined {
                now_vec.push(Point::from_x_y_type((x, y), ptype));
            }
        }

        ret
    }
}
