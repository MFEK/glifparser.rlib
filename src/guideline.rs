use integer_or_float::IntegerOrFloat;

use crate::color::Color;
#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GuidelinePoint {
    pub x: f32,
    pub y: f32
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Guideline {
    pub at: GuidelinePoint,
    pub angle: IntegerOrFloat,
    pub name: Option<String>,
    pub color: Option<Color>,
    pub identifier: Option<String>
}

impl Guideline {
    fn new() -> Self {
        Self {
            at: GuidelinePoint {x: 0., y: 0.},
            angle: IntegerOrFloat::Integer(0),
            name: None,
            color: None,
            identifier: None,
        }
    }

    pub fn from_x_y_angle(x: f32, y: f32, angle: IntegerOrFloat) -> Self {
        let mut ret = Self::new();
        ret.at.x = x;
        ret.at.y = y;
        ret.angle = angle;
        ret
    }

    pub fn from_name_x_y_angle(name: String, x: f32, y: f32, angle: IntegerOrFloat) -> Self {
        let mut ret = Self::from_x_y_angle(x, y, angle);
        ret.name = Some(name);
        ret
    }
}
