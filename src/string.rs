use crate::error::GlifStringConversionError;

use itertools::Itertools as _;

use core::str::FromStr;

#[cfg(feature = "glifserde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "glifserde")]
mod serde_impl;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Deref, DerefMut, AsRef, AsMut, PartialEq, Debug, Display, Default)]
pub struct GlifString(#[cfg_attr(feature = "glifserde", serde(with = "self::serde_impl"))] String);

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Deref, DerefMut, AsRef, AsMut, PartialEq, Debug, Display, Default)]
pub struct GlifStringLenOne(#[cfg_attr(feature = "glifserde", serde(with = "self::serde_impl"))] String);

impl TryFrom<String> for GlifString {
    type Error = GlifStringConversionError;
    fn try_from(s: String) -> Result<Self, GlifStringConversionError> {
        if let Some((idx, c)) = s.chars().find_position(|c|c.is_ascii_control()) {
            Err(GlifStringConversionError::HasControlCharacter{idx, c})
        } else {
            Ok(GlifString(s))
        }
    }
}

impl TryFrom<&String> for GlifString {
    type Error = GlifStringConversionError;
    fn try_from(s: &String) -> Result<Self, GlifStringConversionError> {
        s.to_string().try_into()
    }
}

impl TryFrom<String> for GlifStringLenOne {
    type Error = GlifStringConversionError;
    fn try_from(s: String) -> Result<Self, GlifStringConversionError> {
        let gs: GlifString = s.try_into()?;
        if gs.len() == 0 {
            Err(GlifStringConversionError::LenZero)
        } else {
            Ok(GlifStringLenOne(gs.0))
        }
    }
}

pub trait ToGlifString {
    fn to_glif_string(self) -> GlifString;
}

impl ToGlifString for String {
    fn to_glif_string(mut self) -> GlifString {
        while let Err(GlifStringConversionError::HasControlCharacter{ idx, .. }) = GlifString::try_from(&self) {
            self.remove(idx);
        }
        self.try_into().unwrap()
    }
}

impl FromStr for GlifString {
    type Err = GlifStringConversionError;
    fn from_str(s: &str) -> Result<Self, GlifStringConversionError> {
        s.to_string().try_into()
    }
}
