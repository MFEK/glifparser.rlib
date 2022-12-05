
use serde::{Serialize, Deserialize};
use crate::PointData;
use crate::Point;

use super::contour_operations::{ContourOperations, ContourOperation};
use super::inner::MFEKCommonInner;
use super::inner::MFEKContourInner;
use super::inner::MFEKContourInnerType;
use super::inner::cubic::MFEKCubicInner;
use super::point::MFEKPointCommon;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MFEKContour<PD: PointData> {
    pub inner: MFEKContourInner<PD>,
    pub operation: Option<ContourOperations<PD>>,
}

// A wrapper type that indicates the contour inner is gaurunteed to be an MFEKCubicInner when returned
// from a glifparser or math.rlib function.
pub struct MFEKCubicContour<PD: PointData>(pub MFEKContour<PD>);

impl<PD: PointData> From<&Vec<Point<PD>>> for MFEKContour<PD> {
    fn from(contour: &Vec<Point<PD>>) -> Self {
        Self {
            inner: MFEKContourInner::Cubic( contour.clone() ),
            operation: None,
        }
    }
}

impl<PD: PointData> From<Vec<Point<PD>>> for MFEKContour<PD> {
    fn from(contour: Vec<Point<PD>>) -> Self {
        Self {
            inner: MFEKContourInner::Cubic( contour ),
            operation: None,
        }
    }
}

pub struct MFEKContourCommonIterator<'a, PD: PointData> {
    pub index: usize,
    pub contour: &'a dyn MFEKContourCommon<PD>
}

impl<'a, PD: PointData> Iterator for MFEKContourCommonIterator<'a, PD> {
    type Item = &'a dyn MFEKPointCommon<PD>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.contour.get_point(self.index);
        self.index += 1;
        ret
    }
}

impl<PD: PointData> MFEKContour<PD> {
    pub fn iter(&self) -> MFEKContourCommonIterator<'_, PD>{
        return MFEKContourCommonIterator { index: 0, contour: self }
    }
}

pub struct MFEKCommonMismatchError;

// This is implemented across both the contour and it's inner members.
// Some of this should be split into it's own
pub trait MFEKContourCommon<PD: PointData> {
    fn len(&self) -> usize;
    fn get_type(&self) -> MFEKContourInnerType;


    fn first(&self) -> &dyn MFEKPointCommon<PD>;
    fn last(&self) -> &dyn MFEKPointCommon<PD>;

    fn is_open(&self) -> bool;
    fn is_closed(&self) -> bool;
    fn set_open(&mut self);
    fn set_closed(&mut self);

    fn is_empty(&self) -> bool;

    fn get_point(&self, pidx: usize) -> Option<&dyn MFEKPointCommon<PD>>;
    fn get_point_mut(&mut self, pidx: usize) -> Option<&mut dyn MFEKPointCommon<PD>>;

    // These functions can be used to go from the generic &dyn MFEKContourCommon -> concrete types it's
    // advised to add new functions like these when you add new contour types.
    fn cubic(&self) -> Option<&MFEKCubicInner<PD>>;
    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>>;

    // These modify the contour in place. Anything that returns a new object of the implementing type should
    // use the Outer/Inner traits instead.
    fn delete(&mut self, index: usize);
    fn reverse(&mut self);
}

pub trait MFEKCommonOuter<PD: PointData> {
    fn sub(&self, start_index: usize, end_index: usize) -> MFEKContour<PD>;
    fn append(&mut self, other: &MFEKContour<PD>) -> Result<(), MFEKCommonMismatchError>;
}

impl<PD: PointData> MFEKCommonOuter<PD> for MFEKContour<PD> {
    fn sub(&self, start_index: usize, end_index: usize) -> MFEKContour<PD> {
        let mut result: MFEKContour<PD> = MFEKContour {
            inner: self.inner.sub(start_index, end_index),
            operation: None,
        };

        result.operation = self.operation.clone();
        result.operation.sub(start_index, end_index);

        result
    }

    fn append(&mut self, other: &MFEKContour<PD>) -> Result<(), MFEKCommonMismatchError> {
        let inner_result = self.inner.append(&other.inner);

        if let Ok(_) = inner_result {
            self.operation.append(other)
        }

        inner_result
    }
}

impl<PD: PointData> MFEKContourCommon<PD> for MFEKContour<PD> {
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    fn is_closed(&self) -> bool {
        !self.is_open()
    }

    fn reverse(&mut self) {
        self.inner.reverse()
        //TODO: Contour operations go here.
    }

    fn delete(&mut self, index: usize) {
        self.inner.delete(index);
        self.operation.remove_op(index);
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn set_open(&mut self) {
        self.inner.set_open()
    }

    fn set_closed(&mut self) {
        self.inner.set_closed()
    }

    fn get_point_mut(&mut self, pidx: usize) -> Option<&mut dyn MFEKPointCommon<PD>>{
        self.inner.get_point_mut(pidx)
    }

    fn get_point(&self, pidx: usize) -> Option<&dyn MFEKPointCommon<PD>> {
        self.inner.get_point(pidx)
    }

    fn get_type(&self) -> MFEKContourInnerType {
        self.inner.get_type()
    }

    fn cubic(&self) -> Option<&MFEKCubicInner<PD>> {
        self.inner.cubic()
    }

    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>> {
        self.inner.cubic_mut()
    }

    fn first(&self) -> &dyn MFEKPointCommon<PD> {
        self.get_point(0).unwrap()
    }

    fn last(&self) -> &dyn MFEKPointCommon<PD> {
        self.get_point(self.len()-1).unwrap()
    }
    
}

impl<PD: PointData> MFEKContourCommon<PD> for MFEKCubicContour<PD> {
    fn len(&self) -> usize {
        self.0.inner.len()
    }

    fn is_open(&self) -> bool {
        self.0.inner.is_open()
    }

    fn is_closed(&self) -> bool {
        !self.is_open()
    }

    fn reverse(&mut self) {
        self.0.inner.reverse()
        //TODO: Contour operations go here.
    }

    fn delete(&mut self, index: usize) {
        self.0.inner.delete(index)
        //TODO: Contour operations go here.
    }

    fn is_empty(&self) -> bool {
        self.0.inner.is_empty()
    }

    fn set_open(&mut self) {
        self.0.inner.set_open()
    }

    fn set_closed(&mut self) {
        self.0.inner.set_closed()
    }

    fn get_point_mut(&mut self, pidx: usize) -> Option<&mut dyn MFEKPointCommon<PD>>{
        self.0.inner.get_point_mut(pidx)
    }

    fn get_point(&self, pidx: usize) -> Option<&dyn MFEKPointCommon<PD>> {
        self.0.inner.get_point(pidx)
    }

    fn get_type(&self) -> MFEKContourInnerType {
        self.0.inner.get_type()
    }

    fn cubic(&self) -> Option<&MFEKCubicInner<PD>> {
        self.0.inner.cubic()
    }

    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>> {
        self.0.inner.cubic_mut()
    }

    fn first(&self) -> &dyn MFEKPointCommon<PD> {
        self.0.inner.first()
    }

    fn last(&self) -> &dyn MFEKPointCommon<PD> {
        self.0.inner.last()
    }
    
}


impl<PD: PointData> From<&MFEKContourInner<PD>> for Vec<Point<PD>> {
    fn from(contour: &MFEKContourInner<PD>) -> Vec<Point<PD>> {
        match contour{
            MFEKContourInner::Cubic(contour) => {return contour.clone()}
        }
    }
}

impl<PD: PointData> From<&Vec<Point<PD>>> for MFEKContourInner<PD> {
    fn from(contour: &Vec<Point<PD>>) -> MFEKContourInner<PD> {
        MFEKContourInner::Cubic(contour.clone())
    }
}