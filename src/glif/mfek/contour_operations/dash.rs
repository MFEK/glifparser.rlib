use serde::{Serialize, Deserialize};
use crate::glif::{MFEKContour, PointData};
use super::ContourOperation;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DashCull {
    pub width: f32,
    pub area_cutoff: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DashContour {
    pub stroke_width: f32,
    pub cull: Option<DashCull>,
    pub dash_desc: Vec<f32>,
    pub include_last_path: bool,
    pub paint_join: u8, // skia::PaintJoin,
    pub paint_cap: u8, // skia::PaintCap,
}

impl<PD: PointData> ContourOperation<PD> for DashContour {
    fn sub(&mut self, _begin: usize, _end: usize) {}
    fn append(&mut self, _append: &MFEKContour<PD>) {}
    fn insert_op(&mut self, _point_idx: usize) {}
    fn remove_op(&mut self, _point_idx: usize) {}
}
