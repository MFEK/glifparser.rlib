use serde::{Serialize, Deserialize};

use crate::glif::{MFEKOutline, MFEKContour};
use crate::PointData;

use super::ContourOperation;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternCopies {
    Single,
    Repeated,
    Fixed(usize) // TODO: Implement
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternSubdivide {
    /// no splitting
    Off,
    /// split each curve at its midpoint
    Simple(usize), // The value here is how many times we'll subdivide simply
    // split the input pattern each x degrees in change in direction on the path
    //Angle(f64) TODO: Implement.
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternHandleDiscontinuity {
    /// no handling
    Off,
    /// handle by splitting
    Split(f64) 
    // Cut TODO: implement
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PatternStretch {
    /// no stretching
    Off,
    /// stretch the pattern
    On,
    /// stretch the spacing between the pattern
    Spacing,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PAPContour<PD: PointData> {
    pub pattern: MFEKOutline<PD>,
    pub copies: PatternCopies,
    pub subdivide: PatternSubdivide,
    pub is_vertical: bool, // TODO: Implement this. Might replace it with a general rotation parameter to make it more useful.
    pub stretch: PatternStretch,
    pub spacing: f64,
    pub simplify: bool,
    pub normal_offset: f64,
    pub tangent_offset: f64,
    pub pattern_scale: (f64, f64),
    pub center_pattern: bool,
    pub prevent_overdraw: f64,
    pub two_pass_culling: bool,
    pub reverse_path: bool,
    pub reverse_culling: bool,
}

impl<PD: PointData> ContourOperation<PD> for PAPContour<PD> {
    fn sub(&mut self, _begin: usize, _end: usize) {}
    fn append(&mut self,_append: &MFEKContour<PD>) {}
    fn insert_op(&mut self, _point_idx: usize) {}
    fn remove_op(&mut self, _point_idx: usize) {}
}