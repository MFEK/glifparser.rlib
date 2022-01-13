use crate::point::{FromKurboPoint, ToKurboPoint};

pub trait QuadToCubic<QCO: FromKurboPoint + ToKurboPoint, const N: usize> {
    fn quad_to_cubic(self) -> [QCO; N];
}

impl<QCO: FromKurboPoint + ToKurboPoint> QuadToCubic<QCO, 4> for [QCO; 3] {
    fn quad_to_cubic(self) -> [QCO; 4] {
        #[allow(unused_assignments)]
        let [p0, mut p1, mut p2, p3] = [self[0].to_kurbo(), self[1].to_kurbo(), self[2].to_kurbo(), self[2].to_kurbo()];
        p1.x = p0.x + (2./3.) * (p1.x-p0.x);
        p1.y = p0.y + (2./3.) * (p1.y-p0.y);
        p2.x = p2.x + (2./3.) * (p1.x-p2.x);
        p2.y = p2.y + (2./3.) * (p1.y-p2.y);
        [QCO::from_kurbo(&p0), QCO::from_kurbo(&p1), QCO::from_kurbo(&p2), QCO::from_kurbo(&p3)]
    }
}

// This method of Quad->Cubic conversion is used all over the place in FontForge.
