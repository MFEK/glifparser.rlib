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
#[derive(Clone, Debug, Derivative, PartialEq, Eq, Unwrap, IsVariant)]
#[derivative(Default(new="true"))]
pub enum AnchorType {
    Mark,
    #[derivative(Default)]
    Base,
}

impl<S> From<S> for AnchorType where S: AsRef<str> {
    fn from(s: S) -> Self {
        if s.as_ref().chars().nth(0) == Some('_') {
            Self::Mark
        } else {
            Self::Base
        }
    }
}

pub trait FromOption<S: Into<AnchorType>>: Default {
    fn from_option(s: Option<S>) -> AnchorType {
        match s {
            Some(ss) => ss.into(),
            None => AnchorType::default()
        }
    }
}
impl<S: Into<Self>> FromOption<S> for AnchorType {}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Constructor, Clone, Debug, Default, PartialEq)]
pub struct Anchor<PD: PointData> {
    pub x: f32,
    pub y: f32,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub class: Option<String>,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub data: PD,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub atype: AnchorType,
}

impl<PD: PointData> Anchor<PD> {
    pub fn from_glif(ga: &GlifAnchor, pedantry: Pedantry) -> Result<Self, GlifParserError> {
        let (x, y) = (pedantry.level.maybe_round(ga.x, FloatClass::Anchor), pedantry.level.maybe_round(ga.y, FloatClass::Anchor));
        let class = ga.class.as_ref().map(|gs|gs.to_string());
        if pedantry.mend != Mend::Always && (!ga.x.holding_integer().is_ok() || !ga.y.holding_integer().is_ok()) {
            return Err(GlifParserError::PedanticXmlParseError("Anchor was a float, not an integer!".to_string()))
        }
        let atype = AnchorType::from_option(class.as_ref());
        Ok(Self {
            x, y, class, atype, data: PD::default()
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
