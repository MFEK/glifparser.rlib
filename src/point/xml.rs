//use crate::glif::IntoXML;
use crate::xml::Element;

use super::{GlifPoint, PointType};

impl GlifPoint {
    pub fn xml(&self) -> Element {
        let mut el = Element::new("point");
        el.attributes.insert("x".to_owned(), self.x.to_string());
        el.attributes.insert("y".to_owned(), self.y.to_string());
        let ptype = self.ptype.to_string();
        match ptype.as_str() {
            "offcurve" => None, // while this name is OK, most often not written
            _ => el.attributes.insert("type".to_owned(), ptype),
        };
        match &self.name {
            Some(name) => el.attributes.insert("name".to_owned(), name.to_string()),
            None => None,
        };
        if self.smooth {
            debug_assert!(self.ptype != PointType::OffCurve);
            el.attributes.insert("smooth".to_owned(), "yes".to_owned());
        }
        el
    }
}
