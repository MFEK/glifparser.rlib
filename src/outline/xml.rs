use crate::xml::{Element, IntoXML, XMLNode};

use std::collections::VecDeque;

macro_rules! impl_ixml {
    ($typ:ident) => {
        impl<I: IntoXML + Sized> IntoXML for $typ<I> {
            fn xml(&self) -> Element {
                let mut outline_node = Element::new("outline");
                outline_node.children = self
                    .into_iter()
                    .map(|gc| XMLNode::Element(gc.xml()))
                    .collect();
                outline_node
            }
        }
    }
}
impl_ixml!(Vec);
impl_ixml!(VecDeque);
