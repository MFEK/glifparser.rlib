use integer_or_float::IntegerOrFloat;

use crate::error::GlifParserError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: IntegerOrFloat,
    pub g: IntegerOrFloat,
    pub b: IntegerOrFloat,
    pub a: IntegerOrFloat
}

use std::str::FromStr;
use std::convert::TryFrom;
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

// This follows the UFO spec, not other specs that would say to do e.g. rgba(1,1,1,1)
impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{},{},{},{}", self.r.to_string(), self.g.to_string(), self.b.to_string(), self.a.to_string())
    }
}
