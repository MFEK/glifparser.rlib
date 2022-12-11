use super::kurbo::{FromKurboPoint, KurboPoint, ToKurboPoint};
use crate::point::{IsValid, PointLike};
use integer_or_float::IntegerOrFloat;
use skia::Point as SkPoint;
use skia_safe as skia;

pub trait ToSkiaPoint {
    fn to_skia(&self) -> SkPoint;
}

impl ToKurboPoint for SkPoint {
    fn to_kurbo(&self) -> KurboPoint {
        KurboPoint::new(self.x as f64, self.y as f64)
    }
}

impl ToSkiaPoint for KurboPoint {
    fn to_skia(&self) -> SkPoint {
        SkPoint::new(self.x as f32, self.y as f32)
    }
}

impl FromKurboPoint for SkPoint {
    fn from_kurbo(kp: &KurboPoint) -> SkPoint {
        kp.to_skia()
    }
}

impl IsValid for SkPoint {
    fn is_valid(&self) -> bool {
        !self.x.is_nan() && !self.y.is_nan()
    }
}

impl PointLike for SkPoint {
    fn x(&self) -> IntegerOrFloat {
        IntegerOrFloat::Float(self.x)
    }
    fn y(&self) -> IntegerOrFloat {
        IntegerOrFloat::Float(self.y)
    }
    fn set_x(&mut self, x: IntegerOrFloat) {
        self.set(f32::from(x), self.y)
    }
    fn set_y(&mut self, y: IntegerOrFloat) {
        self.set(self.x, f32::from(y))
    }
}
