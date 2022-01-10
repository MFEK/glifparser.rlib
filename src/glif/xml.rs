pub use xmltree::{
    Element, ParseError as Error,
    XMLNode::{self, CData, Comment, Text},
};

pub use super::read::FromXML;
pub use super::write::{IntoXML, TryIntoXML};

use crate::glif::Glif;
use crate::point::PointData;

pub trait XMLConversion: IntoXML + FromXML {}

impl<PD: PointData> Glif<PD> {
    pub fn advance_xml(&self) -> Element {
        let mut advanceel = Element::new("advance");
        advanceel.attributes.insert("width".to_owned(), self.width.unwrap().to_string());
        advanceel
    }
}
