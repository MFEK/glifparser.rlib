use crate::point::PointData;
use crate::outline::IntoGlifPoints as _;
use crate::xml::{Element, IntoXML, XMLNode};

use super::{Contour, GlifContour};

impl IntoXML for GlifContour {
    fn xml(&self) -> Element {
        let mut contour_node = Element::new("contour");
        contour_node.children = self
            .into_iter()
            .map(|gp| XMLNode::Element(gp.xml()))
            .collect();
        contour_node
    }
}

impl<PD: PointData> IntoXML for Contour<PD> {
    fn xml(&self) -> Element {
        self.clone().into_glifpoints().xml()
    }
    fn into_xml(self) -> Element {
        self.into_glifpoints().xml()
    }
}
