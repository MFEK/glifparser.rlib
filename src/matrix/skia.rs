use skia_safe as skia;
use skia::Matrix;
use kurbo::Affine;

use super::GlifMatrix;

pub trait ToSkiaMatrix {
    fn to_skia_matrix(&self) -> Matrix;
}

impl ToSkiaMatrix for GlifMatrix {
    fn to_skia_matrix(&self) -> Matrix {
        let m: Affine = (*self).into();
        m.to_skia_matrix()
    }
}

impl ToSkiaMatrix for Affine {
    fn to_skia_matrix(&self) -> Matrix {
        Matrix::from_affine(&self.as_coeffs().map(|f|f as f32))
    }
}

impl ToSkiaMatrix for [f64; 6] {
    fn to_skia_matrix(&self) -> Matrix {
        Matrix::from_affine(&self.map(|f|f as f32))
    }
}

impl ToSkiaMatrix for [f32; 6] {
    fn to_skia_matrix(&self) -> Matrix {
        Matrix::from_affine(self)
    }
}
