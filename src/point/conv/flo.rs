#![cfg(feature = "flo_curves")]

use flo_curves as flo;
use flo::BezierCurve;
use crate::point::{Point, PointData, PointType};

impl<PD: PointData> flo::geo::Coordinate2D for Point<PD> {
    fn x(&self) -> f64 { self.x as f64 }
    fn y(&self) -> f64 { self.y as f64 }
}

impl<PD: PointData> flo::geo::Coordinate for Point<PD> {
    fn from_components(c: &[f64]) -> Self {
        Point::from_x_y_type((c[0], c[1]), PointType::Curve)
    }
}

impl flo::geo::Geo for CopyPoint {
    type Point = Point;
}
