use skia_safe as skia;
use skia::{Point as SkPoint};
use super::kurbo::{KurboPoint, FromKurboPoint, ToKurboPoint};

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
