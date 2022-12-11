use crate::glif::point::quad::QPoint;
use crate::glif::inner::State;
use crate::PointData;
use crate::point::PointType;

pub type MFEKQuadInner<PD> = Vec<QPoint<PD>>;

impl<PD: PointData> State for MFEKQuadInner<PD> {
    fn is_open(&self) -> bool {
        return self[0].ptype == PointType::Move
    }
}