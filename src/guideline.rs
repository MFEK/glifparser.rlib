mod xml;

use integer_or_float::IntegerOrFloat;

use crate::color::Color;
use crate::point::PointData;

#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};

use std::fmt::Debug;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GuidelinePoint {
    pub x: f32,
    pub y: f32
}

impl Into<(f32, f32)> for GuidelinePoint {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Guideline<GD: PointData> {
    pub at: GuidelinePoint,
    pub angle: IntegerOrFloat,
    pub name: Option<String>,
    pub color: Option<Color>,
    pub identifier: Option<String>,
    pub data: GD,
}

impl<GD: PointData> Guideline<GD> {
    pub fn as_plist_dict(&self) -> plist::Dictionary {
        let mut dict = plist::Dictionary::new();
        dict.insert("x".to_string(), plist::Value::Real(self.at.x.into()));
        dict.insert("y".to_string(), plist::Value::Real(self.at.y.into()));
        dict.insert("angle".to_string(), plist::Value::Real(self.angle.into()));
        if let Some(ref name) = self.name {
            dict.insert("name".to_string(), plist::Value::String(name.to_string()));
        }
        if let Some(ref color) = self.color {
            dict.insert("color".to_string(), color.as_plist_value());
        }
        dict
    }
}

impl<GD: PointData> Guideline<GD> {
    fn new() -> Self {
        Self::default()
    }

    pub fn from_x_y_angle(x: f32, y: f32, angle: IntegerOrFloat) -> Self {
        let mut ret = Self::new();
        ret.at.x = x;
        ret.at.y = y;
        ret.angle = angle;
        ret
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    pub fn data(mut self, data: impl Into<GD>) -> Self {
        self.data = data.into();
        self
    }
}
