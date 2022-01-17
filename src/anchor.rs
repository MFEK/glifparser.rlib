use integer_or_float::IntegerOrFloat;

mod xml;

use std::fmt::Debug;
#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};

use crate::error::GlifParserError;
use crate::pedantry::{FloatClass, Mend, Pedantry};
use crate::point::PointData;
use crate::string::GlifStringLenOne;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Constructor, Clone, Debug, Default, PartialEq)]
pub struct Anchor<PD: PointData> {
    pub x: f32,
    pub y: f32,
    pub class: Option<String>,
    pub data: PD,
}

impl<PD: PointData> Anchor<PD> {
    pub fn from_glif(ga: &GlifAnchor, pedantry: Pedantry) -> Result<Self, GlifParserError> {
        let (x, y) = (pedantry.level.maybe_round(ga.x, FloatClass::Anchor), pedantry.level.maybe_round(ga.y, FloatClass::Anchor));
        let class = ga.class.as_ref().map(|gs|gs.to_string());
        if pedantry.mend != Mend::Always && (!ga.x.holding_integer().is_ok() || !ga.y.holding_integer().is_ok()) {
            return Err(GlifParserError::PedanticXmlParseError("Anchor was a float, not an integer!".to_string()))
        }
        Ok(Self {
            x, y, class, data: PD::default()
        })
    }
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Constructor, Clone, Debug, Default, PartialEq)]
pub struct GlifAnchor {
    pub x: IntegerOrFloat,
    pub y: IntegerOrFloat,
    pub class: Option<GlifStringLenOne>,
}
