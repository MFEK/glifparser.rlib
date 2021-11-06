use crate::point::{Point, PointData, Handle, GlifPoint};

impl<PD: PointData> From<&Point<PD>> for Handle {
    fn from(p: &Point<PD>) -> Handle {
        Handle::At(p.x, p.y)
    }
}

impl<PD: PointData> From<&GlifPoint> for Point<PD> {
    fn from(gp: &GlifPoint) -> Self {
        Self {
            x: gp.x,
            y: gp.y,
            ptype: gp.ptype,
            name: gp.name.clone(),
            ..Self::default()
        }
    }
}
