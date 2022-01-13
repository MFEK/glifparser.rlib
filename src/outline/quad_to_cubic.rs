use crate::point::{FromKurboPoint, ToKurboPoint};

pub trait QuadToCubic<QCO: FromKurboPoint + ToKurboPoint, const N: usize> {
    fn quad_to_cubic(self) -> [QCO; N];
}

// This method of Quad->Cubic conversion is used all over the place in FontForge.
impl<QCO: FromKurboPoint + ToKurboPoint> QuadToCubic<QCO, 4> for [QCO; 3] {
    fn quad_to_cubic(self) -> [QCO; 4] {
        #[allow(unused_assignments)]
        let [p0_o, p1_o, p2_o, p3_o] = [self[0].to_kurbo(), self[1].to_kurbo(), self[2].to_kurbo(), self[2].to_kurbo()];
        let [p0, mut p1, mut p2, p3] = [p0_o, p1_o, p2_o, p3_o];
        p1.x = p0_o.x + (2./3.) * (p1_o.x-p0_o.x);
        p1.y = p0_o.y + (2./3.) * (p1_o.y-p0_o.y);
        p2.x = p2_o.x + (2./3.) * (p1_o.x-p2_o.x);
        p2.y = p2_o.y + (2./3.) * (p1_o.y-p2_o.y);
        [QCO::from_kurbo(&p0), QCO::from_kurbo(&p1), QCO::from_kurbo(&p2), QCO::from_kurbo(&p3)]
    }
}
