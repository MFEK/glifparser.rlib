use crate::glif::IntoXML;
use crate::xml::Element;

use super::{Guideline, PointData};

impl<GD: PointData> IntoXML for Guideline<GD> {
    fn xml(&self) -> Element {
        let mut guideline_node = xmltree::Element::new("guideline");
        guideline_node.attributes.insert("x".to_string(), self.at.x.to_string());
        guideline_node.attributes.insert("y".to_string(), self.at.y.to_string());
        guideline_node.attributes.insert("angle".to_string(), self.angle.to_string());
        if let Some(c) = self.color {
            guideline_node.attributes.insert("color".to_string(), c.to_string());
        }
        if let Some(n) = &self.name {
            guideline_node.attributes.insert("name".to_string(), n.clone());
        }
        if let Some(i) = &self.identifier {
            guideline_node.attributes.insert("identifier".to_string(), i.clone());
        }
        guideline_node
    }
}
