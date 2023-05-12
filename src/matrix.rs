//! Shared behavior between `<component>`, `<image>` based on PostScript-style matrices of 6 values

#[cfg(feature = "skia")]
pub mod skia;
#[cfg(feature = "skia")]
pub use skia::ToSkiaMatrix;

mod write;
pub(crate) use self::write::write_matrix as write;

use crate::point::{Handle, PointData, Point};
use kurbo::Point as KurboPoint;
use integer_or_float::IntegerOrFloat;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlifMatrix(pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat);

pub use kurbo::Affine;
impl Into<Affine> for GlifMatrix {
    fn into(self) -> Affine {
        Affine::new([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()])
    }
}

pub trait ApplyMatrix {
    fn apply_matrix(&mut self, matrix: Affine);
}

impl<PD: PointData> ApplyMatrix for Point<PD> {
    fn apply_matrix(&mut self, matrix: Affine) {
        let kbp = matrix * KurboPoint::new(self.x as f64, self.y as f64);
        self.x = kbp.x as f32;
        self.y = kbp.y as f32;

        if let Handle::At(ax, ay) = self.a {
            let kbpa = matrix * KurboPoint::new(ax as f64, ay as f64);
            self.a = Handle::At(kbpa.x as f32, kbpa.y as f32);
        }

        if let Handle::At(bx, by) = self.a {
            let kbpb = matrix * KurboPoint::new(bx as f64, by as f64);
            self.b = Handle::At(kbpb.x as f32, kbpb.y as f32);
        }
    }
}
