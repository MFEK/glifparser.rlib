use crate::glif::contour::MFEKContourCommon;
use crate::glif::point::MFEKPointCommon;
use crate::glif::point::quad::QPoint;
use crate::glif::inner::State;
use crate::PointData;
use crate::point::PointType;

use super::MFEKContourInnerType;
use super::cubic::MFEKCubicInner;
use super::hyper::MFEKHyperInner;

pub type MFEKQuadInner<PD> = Vec<QPoint<PD>>;

impl<PD: PointData> State for MFEKQuadInner<PD> {
    fn is_open(&self) -> bool {
        return self[0].ptype == PointType::Move
    }
}

impl<PD: PointData> MFEKContourCommon<PD> for MFEKQuadInner<PD> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get_type(&self) -> MFEKContourInnerType {
        MFEKContourInnerType::Quad
    }

    fn is_open(&self) -> bool {
        State::is_open(self)
    }

    fn is_closed(&self) -> bool {
        State::is_closed(self)
    }

    fn set_open(&mut self) {
        self[0].ptype = PointType::Move
    }

    fn set_closed(&mut self) {
        self[0].ptype = PointType::Curve
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn get_point(&self, pidx: usize) -> Option<&dyn MFEKPointCommon<PD>> {
        if let Some(hp) = self.get(pidx) {
            Some(hp)
        } else {
            None
        }    
    }

    fn get_point_mut(&mut self, pidx: usize) -> Option<&mut dyn MFEKPointCommon<PD>> {        
        if let Some(hp) = self.get_mut(pidx) {
            Some(hp)
        } else {
            None
        }
    }

    fn delete(&mut self, index: usize) {
        self.remove(index);
    }

    fn reverse_points(&mut self) {
        self.reverse();
    }

    fn quad(&self) -> Option<&MFEKQuadInner<PD>> {
        Some(self)
    }

    fn quad_mut(&mut self) -> Option<&mut MFEKQuadInner<PD>> {
        Some(self)
    }

    fn cubic(&self) -> Option<&MFEKCubicInner<PD>> {
        None
    }

    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>> {
        None
    }

    fn hyper(&self) -> Option<&MFEKHyperInner<PD>> {
        None
    }

    fn hyper_mut(&mut self) -> Option<&mut MFEKHyperInner<PD>> {
        None
    }
}