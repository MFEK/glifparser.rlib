//! impl's/struct for shared `<image>`/`<guideline>`/layer color behavior

use integer_or_float::IntegerOrFloat;

use crate::error::GlifParserError;
#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: IntegerOrFloat,
    pub g: IntegerOrFloat,
    pub b: IntegerOrFloat,
    pub a: IntegerOrFloat
}

impl Color {
    pub fn from_rgba(r: IntegerOrFloat, g: IntegerOrFloat, b: IntegerOrFloat, a: IntegerOrFloat) -> Color {
        Color { r, g, b, a }
    }

    pub fn as_plist_value(&self) -> plist::Value {
        plist::Value::Array(vec![plist::Value::Real(self.r.into()), plist::Value::Real(self.g.into()), plist::Value::Real(self.b.into()), plist::Value::Real(self.a.into())])
    }
}

use std::str::FromStr;
use std::convert::TryFrom;

/// This follows the UFO spec, e.g. `"0,0,1,1"` → blue at 100% opacity
impl FromStr for Color {
    type Err = GlifParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut to_parse = s.to_string();
        to_parse.retain(|c| !c.is_whitespace());
        let numbers: Vec<_> = s.split(',').collect();
        if let Some(&[rr, gg, bb, aa]) = numbers.chunks(4).next() {
            let convert_to_iof = |c: &str| -> Result<IntegerOrFloat, GlifParserError> {Ok(IntegerOrFloat::try_from(c).or(Err(GlifParserError::ColorNotRGBA))?)};
            let r: IntegerOrFloat = convert_to_iof(rr)?;
            let g: IntegerOrFloat = convert_to_iof(gg)?;
            let b: IntegerOrFloat = convert_to_iof(bb)?;
            let a: IntegerOrFloat = convert_to_iof(aa)?;
            Ok(Color{r, g, b, a})
        } else {
            Err(GlifParserError::ColorNotRGBA)
        }
    }
}

/// This follows the UFO spec, not other specs that would say to do e.g. rgba(1,1,1,1)
impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{},{},{},{}", self.r.to_string(), self.g.to_string(), self.b.to_string(), self.a.to_string())
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r.into(), self.g.into(), self.b.into(), self.a.into()]
    }
}

impl From<[f32; 4]> for Color {
    fn from(c: [f32; 4]) -> Self {
        Color::from_rgba(c[0].into(), c[1].into(), c[2].into(), c[3].into())
    }
}
