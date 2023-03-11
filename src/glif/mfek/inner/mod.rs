pub mod cubic;
pub mod quad;
pub mod hyper;

use serde::{Serialize, Deserialize};
use crate::{PointData, contour::State, Point};
use self::{cubic::MFEKCubicInner, quad::MFEKQuadInner, hyper::MFEKHyperInner};

use super::{contour::{MFEKContourCommon, MFEKContourCommonIterator, MFEKCommonMismatchError}, point::{MFEKPointCommon, quad::QPoint}};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MFEKContourInner<PD: PointData> {
    Cubic(MFEKCubicInner<PD>),
    Quad(MFEKQuadInner<PD>),
    Hyper(MFEKHyperInner<PD>)
}

impl<PD: PointData> MFEKContourInner<PD> {
    pub fn as_dyn(&self) -> &dyn MFEKContourCommon<PD> {
        match self {
            MFEKContourInner::Cubic(c) => c,
            MFEKContourInner::Quad(c) => c,
            MFEKContourInner::Hyper(c) => c,
        }
    }

    fn as_dyn_mut(&mut self) -> &mut dyn MFEKContourCommon<PD> {
        match self {
            MFEKContourInner::Cubic(c) => c,
            MFEKContourInner::Quad(c) => c,
            MFEKContourInner::Hyper(c) => c,
        }
    }
}

pub trait MFEKCommonInner<PD: PointData> {
    fn sub(&self, start_index: usize, end_index: usize) -> MFEKContourInner<PD>;
    fn append(&mut self, other: &MFEKContourInner<PD>) -> Result<(), MFEKCommonMismatchError>;
}

impl<PD: PointData> MFEKContourInner<PD> {
    pub fn iter(&self) -> MFEKContourCommonIterator<'_, PD>{
        return MFEKContourCommonIterator { index: 0, contour: self }
    }
}

impl<PD: PointData> MFEKCommonInner<PD> for MFEKContourInner<PD> {
    fn sub(&self, start_index: usize, end_index: usize) -> MFEKContourInner<PD> {
        match self {
            MFEKContourInner::Cubic(contour) => {
                let sub_contour:Vec<Point<PD>> = contour[start_index..end_index].to_vec();
                MFEKContourInner::Cubic(sub_contour)
            }
            MFEKContourInner::Quad(contour) => {
                let sub_contour:Vec<QPoint<PD>> = contour[start_index..end_index].to_vec();
                MFEKContourInner::Quad(sub_contour)
            },
            MFEKContourInner::Hyper(contour) => {
                let sub_contour = contour.get_points()[start_index..end_index].to_vec();
                MFEKContourInner::Hyper(MFEKHyperInner::new(sub_contour, self.is_open()))
            },
        }
    }

    fn append(&mut self, other: &MFEKContourInner<PD>) -> Result<(), MFEKCommonMismatchError> {
        match self {
            MFEKContourInner::Cubic(contour) => {
                if let Some(other_cubic) = other.cubic() {
                    for point in other_cubic {
                        contour.push(point.clone());
                    }
                    Ok(())
                } else {
                    Err(MFEKCommonMismatchError)
                }
            },
            MFEKContourInner::Quad(contour) => {
                if let Some(quad) = other.quad() {
                    for point in quad {
                        contour.push(point.clone());
                    }
                    Ok(())
                } else {
                    Err(MFEKCommonMismatchError)
                }
            },
            MFEKContourInner::Hyper(contour) => {
                if let Some(other_hyper) = other.hyper() {
                    for point in other_hyper.get_points() {
                        contour.get_points_mut().push(point.clone());
                    }
                    Ok(())
                } else {
                    Err(MFEKCommonMismatchError)
                }
            }
        }
    }
}

impl<PD: PointData> MFEKContourCommon<PD> for MFEKContourInner<PD> {
    fn len(&self) -> usize {
        self.as_dyn().len()
    }

    fn is_open(&self) -> bool {
        self.as_dyn().is_open()
    }

    fn is_closed(&self) -> bool {
        !self.is_open()
    }

    fn reverse_points(&mut self) {
        self.as_dyn_mut().reverse_points()
    }

    fn delete(&mut self, index: usize) {
        self.as_dyn_mut().delete(index)
    }

    fn is_empty(&self) -> bool {
        self.as_dyn().is_empty()
    }

    fn set_open(&mut self) {
        self.as_dyn_mut().set_open()
    }

    fn set_closed(&mut self) {
        self.as_dyn_mut().set_closed()
    }

    fn get_point_mut(&mut self, pidx: usize) -> Option<&mut dyn MFEKPointCommon<PD>>{
        self.as_dyn_mut().get_point_mut(pidx)
    }

    fn get_point(&self, pidx: usize) -> Option<&dyn MFEKPointCommon<PD>> {
        self.as_dyn().get_point(pidx)   
    }

    fn get_type(&self) -> MFEKContourInnerType {
        self.as_dyn().get_type()
    }

    fn cubic(&self) -> Option<&MFEKCubicInner<PD>> {
       self.as_dyn().cubic()
    }

    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>> {
        self.as_dyn_mut().cubic_mut()
    }

    fn quad(&self) -> Option<&MFEKQuadInner<PD>> {
        self.as_dyn().quad()
    }

    fn quad_mut(&mut self) -> Option<&mut MFEKQuadInner<PD>> {
        self.as_dyn_mut().quad_mut()
    }

    fn hyper(&self) -> Option<&MFEKHyperInner<PD>> {
        self.as_dyn().hyper()
    }

    fn hyper_mut(&mut self) -> Option<&mut MFEKHyperInner<PD>> {
        self.as_dyn_mut().hyper_mut()
    }
    
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MFEKContourInnerType {
    Cubic,
    Quad,
    Hyper,
}