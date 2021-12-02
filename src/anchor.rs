use std::fmt::Debug;
#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};

use crate::point::PointData;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Anchor<PD: PointData> {
    pub x: f32,
    pub y: f32,
    pub class: String,
    pub r#type: AnchorType,
    pub data: PD,
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AnchorType {
    Undefined,
    Mark,
    Base,
    MarkMark,
    MarkBase,
} // Undefined used everywhere for now as getting type requires parsing OpenType features, which we will be using nom to do since I have experience w/it.

impl<PD: PointData> Anchor<PD> {
    pub fn new() -> Self {
        Anchor {
            x: 0.,
            y: 0.,
            r#type: AnchorType::Undefined,
            class: String::new(),
            data: PD::default(),
        }
    }
}

