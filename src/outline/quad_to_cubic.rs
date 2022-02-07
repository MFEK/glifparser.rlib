use crate::point::PointLike;

pub trait QuadToCubic<QCO: Default, const N: usize> {
    fn quad_to_cubic(self) -> [QCO; N];
}

// This method of Quad->Cubic conversion is used all over the place in FontForge.
impl<QCO> QuadToCubic<QCO, 4> for [QCO; 3] where QCO: PointLike + Default {
    fn quad_to_cubic(self) -> [QCO; 4] {
        let [p0_o, p1_o, p2_o] = self;
        let (mut p1, mut p2): (QCO, QCO) = Default::default();
        p1.set_x( p0_o.x() + (2./3.) * (p1_o.x()-p0_o.x()) );
        p1.set_y( p0_o.y() + (2./3.) * (p1_o.y()-p0_o.y()) );
        p2.set_x( p2_o.x() + (2./3.) * (p1_o.x()-p2_o.x()) );
        p2.set_y( p2_o.y() + (2./3.) * (p1_o.y()-p2_o.y()) );
        let (p0, p3) = (p0_o, p2_o);
        [p0, p1, p2, p3]
    }
}
