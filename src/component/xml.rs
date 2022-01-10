use crate::matrix;
use crate::xml::{Element, IntoXML};

use super::GlifComponent;

impl IntoXML for GlifComponent {
    fn xml(&self) -> Element {
        let mut component_node = Element::new("component");
        component_node
            .attributes
            .insert("base".to_string(), self.base.clone());
        matrix::write!(component_node, self);
        component_node
    }
}
