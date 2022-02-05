/// Kurbo module — warning: only guaranteed to round trip closed contours!

use float_cmp::ApproxEq as _;
use kurbo::{BezPath, PathEl, PathEl::*};

use super::{Contour, Outline, conv::PenOperations};
use crate::error::GlifParserError;
use crate::point::{Handle, Point, PointData, PointType, PointType::*};
use crate::point::PointLike;

use std::iter::Iterator;

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

impl Into<PathEl> for PenOperations {
    fn into(self) -> PathEl {
        match self {
            PenOperations::MoveTo(gp) => PathEl::MoveTo(gp.as_kpoint()),
            PenOperations::LineTo(gp) => PathEl::LineTo(gp.as_kpoint()),
            PenOperations::QuadTo(gpa, gp) => PathEl::QuadTo(gpa.as_kpoint(), gp.as_kpoint()),
            PenOperations::CurveTo(gpa, gp2b, gp) => PathEl::CurveTo(gpa.as_kpoint(), gp2b.as_kpoint(), gp.as_kpoint()),
            PenOperations::Close => PathEl::ClosePath,
        }
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

use crate::outline::conv::IntoPenOperations;
impl<PD: PointData> IntoKurbo for Contour<PD> {
    fn into_kurbo_vec(mut self) -> Result<Vec<PathEl>, GlifParserError> {
        let ret = self
            .into_pen_operations()?
            .into_iter()
            .map(|po| po.into())
            .collect();

        Ok(ret)
    }
}

pub trait FromKurbo {
    fn from_kurbo(kpath: &BezPath) -> Self;
}

trait SplitKurboPath {
    fn split_kurbo_path(&self) -> Vec<Vec<(PointType, Vec<kurbo::Point>)>>;
}

trait IntoKurboPointsVec {
    fn into_kpoint_vec(&self) -> Vec<kurbo::Point>;
}

impl IntoKurboPointsVec for PathEl {
    fn into_kpoint_vec(&self) -> Vec<kurbo::Point> {
        match self {
            MoveTo(kp) => vec![*kp],
            LineTo(kp) => vec![*kp],
            QuadTo(kpa, kp) => vec![*kp, *kpa],
            CurveTo(kpa, kpb, kp) => vec![*kp, *kpb, *kpa],
            ClosePath => vec![],
        }
    }
}
impl SplitKurboPath for BezPath {
    fn split_kurbo_path(&self) -> Vec<Vec<(PointType, Vec<kurbo::Point>)>> {
        let mut koutline = vec![];
        let mut kcontour = vec![];
        // split a kurbo path into its constituent contours
        let iterable: Vec<_> = if self.iter().last().unwrap() != ClosePath {
            self.into_iter().chain([ClosePath].into_iter()).collect()
        } else {
            self.into_iter().collect()
        };
        let mut last_type = None;
        for p in iterable {
            let ptype: PointType = p.into();
            let kpv = p.into_kpoint_vec();
            if ptype == PointType::Move {
                if kcontour.len() > 0 {
                    koutline.push(kcontour);
                }
                kcontour = vec![(ptype, kpv)];
            } else if kpv.len() > 0 {
                kcontour.push((ptype, kpv));
            } else {
                let lp = kcontour.last().unwrap().clone().1;
                let mut rm = kcontour.remove(0);
                let _fp = kcontour.first().unwrap().clone().1;
                if rm.1[0].x.approx_eq(lp[0].x, (f32::EPSILON as f64, 4)) && rm.1[0].y.approx_eq(lp[0].y, (f32::EPSILON as f64, 4)) {
                    rm.0 = PointType::Curve;
                    kcontour[0].0 = PointType::Curve;
                    kcontour.insert(0, rm);
                } else {
                    rm.0 = PointType::Line;
                    kcontour.insert(0, rm);
                }
            }
            last_type = Some(p);
        }

        if kcontour.len() > 0 {
            koutline.push(kcontour);
        }

        koutline
    }
}

impl<PD: PointData> FromKurbo for Outline<PD> {
    fn from_kurbo(kpath: &BezPath) -> Self {
        let mut ret: Outline<PD> = Outline::new();
        let koutline = kpath.split_kurbo_path();

        for skc in koutline.iter() {
            let skc_len = skc.len();
            let mut contour: Contour<PD> = Contour::new();
            let mut next_points;
            for (i, (ptype, points)) in skc.iter().enumerate() {
                if i != skc_len - 1 {
                    next_points = &skc[i+1];
                } else {
                    next_points = &skc[0];
                }

                let mut point = Point::<PD> {
                    name: None,
                    data: None,
                    x: points[0].x as f32,
                    y: points[0].y as f32,
                    smooth: false,
                    // These will be fixed below, if needed
                    a: Handle::Colocated,
                    b: Handle::Colocated,
                    ptype: *ptype,
                };

                match ptype {
                    PointType::Move => {},
                    PointType::Curve => {
                        if next_points.1.len() == 3 {
                            point.a = Handle::At(next_points.1[2].x as f32, next_points.1[2].y as f32);
                        }
                        if let Some(p) = points.get(1) {
                            point.b = Handle::At(p.x as f32, p.y as f32);
                        } else {
                            log::warn!("Expected a next handle that does not exist")
                        }
                    },
                    PointType::Line => {
                        if next_points.1.len() == 3 {
                            // Lines aren't allowed to have off-curve points in glif format
                            point.ptype = PointType::Curve;
                            point.a = Handle::At(next_points.1[2].x as f32, next_points.1[2].y as f32);
                        }
                    },
                    _ => unreachable!("")
                }
                contour.push(point);
            }

            let first = contour.first().unwrap();
            let last = contour.last().unwrap();
            let (x, y, a, b) = (last.x, last.y, last.a, last.b);
            if contour.len() >= 2 && x == first.x && y == first.y {
                let rm = contour.pop().unwrap();
                contour.first_mut().unwrap().b = b;
                //contour.first_mut().unwrap().b = a;
            }

            ret.push(contour);
        }

        ret
    }
}
