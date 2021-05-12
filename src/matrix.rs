#[cfg(feature = "skia")]
pub mod skia;
#[cfg(feature = "skia")]
pub use skia::ToSkiaMatrix;

use integer_or_float::IntegerOrFloat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlifMatrix(pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat, pub IntegerOrFloat);

use kurbo::Affine;
impl Into<Affine> for GlifMatrix {
    fn into(self) -> Affine {
        Affine::new([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()])
    }
}
