pub mod vws;
pub mod pap;
pub mod dash;

use serde::{Serialize, Deserialize};
use self::{vws::VWSContour, pap::PAPContour, dash::DashContour};
use crate::{PointData, glif::MFEKContour};

use super::{pointdata::MFEKPointData, MFEKOutline};

pub fn unknown_op() {
    log::warn!("Found unknown contour operation attached to contour. File was generated with newer MFEKglif, please upgrade to edit properly.");
}

pub fn unknown_op_outline() -> MFEKOutline<MFEKPointData> {
    unknown_op();
    MFEKOutline::new()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ContourOperations<PD: PointData> {
    VariableWidthStroke { data: VWSContour },
    PatternAlongPath { data: PAPContour<PD> },
    DashAlongPath { data: DashContour },
}

pub trait ContourOperation<PD: PointData> {
    fn sub(&mut self, begin: usize, end: usize);
    fn append(&mut self, append: &MFEKContour<PD>);
    fn remove_op(&mut self, idx: usize);
    fn insert_op(&mut self, idx: usize);
}


impl<PD: PointData> ContourOperation<PD> for Option<ContourOperations<PD>> {
    fn sub(&mut self, begin: usize, end: usize) {
        if let Some(op) = self.as_mut() {
            match op {
                ContourOperations::VariableWidthStroke { ref mut data } => {
                    <VWSContour as ContourOperation<PD>>::sub(data, begin, end);
                }
                ContourOperations::PatternAlongPath { ref mut data } => {
                    data.sub(begin, end)
                }
                ContourOperations::DashAlongPath { ref mut data } => <DashContour as ContourOperation<PD>>::sub(data, begin, end),
            }
        }
    }

    fn append(&mut self, append: &MFEKContour<PD>) {
        if let Some(op) = self.as_mut() {
            match op {
                ContourOperations::VariableWidthStroke { ref mut data } => {
                    data.append(append)
                }
                ContourOperations::PatternAlongPath { ref mut data } => {
                    data.append(append)
                }
                ContourOperations::DashAlongPath { ref mut data } => data.append(append),
            }
        }
    }

    fn insert_op(&mut self, idx: usize) {
        if let Some(op) = self.as_mut() {
            match op {
                ContourOperations::VariableWidthStroke { ref mut data } => {
                    <VWSContour as ContourOperation<PD>>::insert_op(data, idx)
                }
                ContourOperations::PatternAlongPath { ref mut data } => data.insert_op(idx),
                ContourOperations::DashAlongPath { ref mut data } => <DashContour as ContourOperation<PD>>::insert_op(data, idx),
            }
        }
    }

    fn remove_op(&mut self, idx: usize) {
        if let Some(op) = self.as_mut() {
            match op {
                ContourOperations::VariableWidthStroke { ref mut data } => {
                    <VWSContour as ContourOperation<PD>>::remove_op(data, idx)
                }
                ContourOperations::PatternAlongPath { ref mut data } => data.remove_op(idx),
                ContourOperations::DashAlongPath { ref mut data } => <DashContour as ContourOperation<PD>>::remove_op(data, idx),
            }
        }
    }
}
