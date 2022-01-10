use itertools::Itertools as _;
use kurbo::{BezPath, PathEl, Shape as _};

use std::collections::{HashSet, VecDeque};

use crate::error::GlifParserError;
use crate::outline;
use crate::outline::GlifOutline;
use crate::point::{GlifPoint, Handle, Point, PointType, PointData, WhichHandle};
use super::{Contour, Outline};

use crate::outline::contour::{PrevNext as _, State as _};
use super::RefigurePointTypes as _;

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
            ClosePath => QClose
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

impl<PD: PointData> FromKurbo for Outline<PD> {
    type Output = Self;
    fn from_kurbo(kpath: &BezPath) -> Self {
        let (mut ptypes, mut glifpoints): (VecDeque<PointType>, Vec<GlifPoint>) = (VecDeque::new(), vec![]);

        let mut prev = None;
        for el in kpath.path_elements(1.0) {
            if prev.map(|elo|elo==ClosePath).unwrap_or(false) && el == ClosePath {
                prev = Some(el);
                continue
            }
            ptypes.push_back(PointType::from(&el));
            let pvec = match el {
                MoveTo(kp) | LineTo(kp) => vec![kp],
                QuadTo(kpb, kp) => vec![kpb, kp],
                CurveTo(kpa, kpb, kp) => vec![kpa, kpb, kp],
                ClosePath => vec![kurbo::Point::new(f64::NAN, f64::NAN)],
            };

            for (i, p) in pvec.iter().enumerate() {
                let to_push = if pvec.len() - 1 == i { GlifPoint::from_kurbo(*p, el.into()) } else { GlifPoint::from_kurbo_offcurve(*p) };
                glifpoints.push(to_push);
            }
            prev = Some(el);
        }
        let gllen = glifpoints.len();

        let mut positions: Vec<_> = glifpoints.iter().positions(|gp|gp.ptype==PointType::QClose).collect();
        let mut glifoutline: GlifOutline = vec![];
        let mut last_pos = 0;
        for pos in positions.iter().chain([&gllen]).peekable() {
            let mut glifcontour = glifpoints[last_pos .. *pos].to_vec();
            if let Some(closest_move) = glifcontour.iter().rposition(|gp|gp.ptype==PointType::Move) {
                glifcontour[closest_move].ptype = PointType::Curve;
            }
            glifoutline.push(glifcontour);
            last_pos = *pos;
        }
        //eprintln!("{:?} {:?} {:?} {:?}", &positions, &glifoutline, &glifpoints, last_pos);
        outline::create::cubic_outline(&glifoutline)
    }
}
