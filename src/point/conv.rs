mod flo;
pub mod kurbo;
#[cfg(feature = "skia")]
mod skia;

use crate::point::{GlifPoint, Handle, Point, PointData, PointType, WhichHandle};

impl<PD: PointData> From<&Point<PD>> for Handle {
    fn from(p: &Point<PD>) -> Handle {
        Handle::At(p.x, p.y)
    }
}

impl GlifPoint {
    pub fn from_handle<PD: PointData>(point: &Point<PD>, wh: WhichHandle) -> Self {
        let (x, y) = point.handle_or_colocated(wh, |f|f, |f|f);
        GlifPoint::from_x_y_type((x, y), PointType::OffCurve)
    }
}

/// Warning: you lose handles with this.
impl<PD: PointData> From<&Point<PD>> for GlifPoint {
    fn from(p: &Point<PD>) -> GlifPoint {
        Self {
            x: p.x.into(),
            y: p.y.into(),
            ptype: p.ptype,
            name: p.name.clone(),
            smooth: p.smooth,
            ..Self::default()
        }
    }
}

impl<PD: PointData> From<&GlifPoint> for Point<PD> {
    fn from(gp: &GlifPoint) -> Self {
        Self {
            x: gp.x.into(),
            y: gp.y.into(),
            ptype: gp.ptype,
            name: gp.name.clone(),
            smooth: gp.smooth,
            ..Self::default()
        }
    }
}
