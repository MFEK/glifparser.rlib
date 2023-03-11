use serde::{Serialize, Deserialize};
use crate::{glif::{point::{hyper::HyperPoint, MFEKPointCommon}, contour::MFEKContourCommon}, PointData};

use super::{MFEKContourInnerType, cubic::MFEKCubicInner, quad::MFEKQuadInner};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MFEKHyperInner<PD: PointData> {
    points: Vec<HyperPoint<PD>>,
    open: bool,
}

impl<PD: PointData> MFEKHyperInner<PD> {
    pub fn new(points: Vec<HyperPoint<PD>>, open: bool) -> Self {
        Self {
            points,
            open,
        }
    }

    pub fn get_points(&self) -> &Vec<HyperPoint<PD>> {
        &self.points
    }

    pub fn get_points_mut(&mut self) -> &mut Vec<HyperPoint<PD>> {
        &mut self.points
    }

    pub fn next_point(&self, idx: usize) -> Option<&HyperPoint<PD>> {
        if idx+1 == self.len() && self.is_closed() && self.len() > 1 {
            self.points.get(0)
        } else {
            self.points.get(idx+1)
        }
    }
}

impl<PD: PointData> MFEKContourCommon<PD> for MFEKHyperInner<PD> {
    fn len(&self) -> usize {
        self.points.len()
    }

    fn get_type(&self) -> MFEKContourInnerType {
        MFEKContourInnerType::Hyper
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn is_closed(&self) -> bool {
        !self.open
    }

    fn set_open(&mut self) {
        self.open = true;
    }

    fn set_closed(&mut self) {
        self.open = false;
    }

    fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    fn get_point(&self, pidx: usize) -> Option<&dyn MFEKPointCommon<PD>> {
        if let Some(hp) = self.points.get(pidx) {
            Some(hp)
        } else {
            None
        }    
    }

    fn get_point_mut(&mut self, pidx: usize) -> Option<&mut dyn MFEKPointCommon<PD>> {        
        if let Some(hp) = self.points.get_mut(pidx) {
            Some(hp)
        } else {
            None
        }
    }

    fn hyper(&self) -> Option<&MFEKHyperInner<PD>> {
        Some(self)
    }

    fn hyper_mut(&mut self) -> Option<&mut MFEKHyperInner<PD>> {
        Some(self)
    }

    fn delete(&mut self, index: usize) {
        self.points.remove(index);
    }

    fn reverse_points(&mut self) {
        self.points.reverse();
    }

    fn cubic(&self) -> Option<&MFEKCubicInner<PD>> {
        None
    }

    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>> {
        None
    }

    fn quad(&self) -> Option<&MFEKQuadInner<PD>> {
        None
    }

    fn quad_mut(&mut self) -> Option<&mut MFEKQuadInner<PD>> {
        None
    }

}