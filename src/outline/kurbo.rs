/// Kurbo module — warning: only guaranteed to round trip closed contours!
use kurbo::{BezPath, PathEl, PathEl::*};

use super::{
    conv::{PenOperations, SplitPenOperations as _},
    Contour, Outline, ToOutline as _,
};
use crate::error::GlifParserError;
use crate::point::PointLike as _;
use crate::point::{GlifPoint, PointData, PointType, PointType::*};

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

impl<'a> From<&'a PenOperations> for PointType {
    fn from(po: &PenOperations) -> Self {
        match po {
            PenOperations::MoveTo(..) => Move,
            PenOperations::LineTo(..) => Line,
            PenOperations::QuadTo(..) => QCurve,
            PenOperations::CurveTo(..) => Curve,
            PenOperations::Close => QClose,
        }
    }
}

impl Into<PathEl> for PenOperations {
    fn into(self) -> PathEl {
        match self {
            PenOperations::MoveTo(gp) => PathEl::MoveTo(gp.as_kpoint()),
            PenOperations::LineTo(gp) => PathEl::LineTo(gp.as_kpoint()),
            PenOperations::QuadTo(gpa, gp) => PathEl::QuadTo(gpa.as_kpoint(), gp.as_kpoint()),
            PenOperations::CurveTo(gpa, gp2b, gp) => {
                PathEl::CurveTo(gpa.as_kpoint(), gp2b.as_kpoint(), gp.as_kpoint())
            }
            PenOperations::Close => PathEl::ClosePath,
        }
    }
}

impl From<PathEl> for PenOperations {
    fn from(el: PathEl) -> PenOperations {
        match el {
            PathEl::MoveTo(gp) => PenOperations::MoveTo(GlifPoint::from_x_y_type(
                (gp.x as f32, gp.y as f32),
                PointType::Move,
            )),
            PathEl::LineTo(gp) => PenOperations::LineTo(GlifPoint::from_x_y_type(
                (gp.x as f32, gp.y as f32),
                PointType::Line,
            )),
            PathEl::QuadTo(gpa, gp) => PenOperations::QuadTo(
                GlifPoint::from_x_y_type((gp.x as f32, gp.y as f32), PointType::OffCurve),
                GlifPoint::from_x_y_type((gpa.x as f32, gpa.y as f32), PointType::QCurve),
            ),
            PathEl::CurveTo(gpa, gp2b, gp) => PenOperations::CurveTo(
                GlifPoint::from_x_y_type((gp.x as f32, gp.y as f32), PointType::OffCurve),
                GlifPoint::from_x_y_type((gp2b.x as f32, gp2b.y as f32), PointType::OffCurve),
                GlifPoint::from_x_y_type((gpa.x as f32, gpa.y as f32), PointType::Curve),
            ),
            PathEl::ClosePath => PenOperations::Close,
        }
    }
}

/// Type (most useful on [`Outline`]) to [`kurbo::BezPath`]
pub trait IntoKurbo: Sized {
    /// Implemented via [`crate::outline::IntoPenOperations`].
    fn into_kurbo(self) -> Result<BezPath, GlifParserError> {
        Ok(BezPath::from_vec(self.into_kurbo_vec()?))
    }
    /// In case you want to use [`PenOperations`].
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

/// [`kurbo::BezPath`] to type (most useful for [`Outline`])
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

impl<PD: PointData> FromKurbo for Outline<PD> {
    fn from_kurbo(kpath: &BezPath) -> Self {
        let gpself: Vec<PenOperations> = kpath.iter().map(|pe| pe.into()).collect();
        let koutline = gpself.split_pen_operations();
        koutline.to_outline()
    }
}
