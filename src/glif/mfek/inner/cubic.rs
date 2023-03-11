use crate::{Point, glif::{contour::MFEKContourCommon, point::MFEKPointCommon}, PointData, contour::State, PointType};

use super::{hyper::MFEKHyperInner, MFEKContourInnerType, quad::MFEKQuadInner};

pub type MFEKCubicInner<PD> = Vec<Point<PD>>;

impl<PD: PointData> MFEKContourCommon<PD> for MFEKCubicInner<PD> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get_type(&self) -> MFEKContourInnerType {
        MFEKContourInnerType::Cubic
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

    fn cubic(&self) -> Option<&MFEKCubicInner<PD>> {
        Some(self)
    }

    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>> {
        Some(self)
    }

    fn hyper(&self) -> Option<&MFEKHyperInner<PD>> {
        None
    }

    fn hyper_mut(&mut self) -> Option<&mut MFEKHyperInner<PD>> {
        None
    }

    fn quad(&self) -> Option<&MFEKQuadInner<PD>> {
        None
    }

    fn quad_mut(&mut self) -> Option<&mut MFEKQuadInner<PD>> {
        None
    }

}