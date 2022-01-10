use crate::glif::IntoXML;
use crate::xml::Element;

use super::{Anchor, PointData};

impl<PD: PointData> IntoXML for Anchor<PD> {
    fn xml(&self) -> Element {
        let mut anchor_node = xmltree::Element::new("anchor");
        anchor_node.attributes.insert("x".to_owned(), self.x.to_string());
        anchor_node.attributes.insert("y".to_owned(), self.y.to_string());
        anchor_node.attributes.insert("name".to_owned(), self.class.to_string());
        anchor_node
    }
}
