pub use kurbo::Point as KurboPoint;
use crate::point::{GlifPoint, PointLike, PointType};

impl<'a> Into<KurboPoint> for &'a dyn PointLike {
    fn into(self) -> KurboPoint {
        KurboPoint{ x: self.x64(), y: self.y64() }
    }
}

pub trait FromKurboPoint {
    fn from_kurbo(kp: &kurbo::Point) -> Self;
}

pub trait ToKurboPoint {
    fn to_kurbo(&self) -> KurboPoint;
}

impl GlifPoint {
    pub fn from_kurbo(kp: kurbo::Point, pt: PointType) -> Self {
        Self::from_x_y_type((kp.x as f32, kp.y as f32), pt)
    }

    pub fn from_kurbo_offcurve(kp: kurbo::Point) -> Self {
        Self::from_kurbo(kp, PointType::OffCurve)
    }
}
