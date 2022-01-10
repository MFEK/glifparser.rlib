use crate::glif::IntoXML;
use crate::xml::Element;

pub trait Codepoint {
    fn display(&self) -> String;
}

impl Codepoint for char {
    fn display(&self) -> String {
        format!("{:X}", *self as u32)
    }
}

impl IntoXML for dyn Codepoint {
    fn xml(&self) -> Element {
        let mut unicode = xmltree::Element::new("unicode");
        unicode.attributes.insert("hex".to_owned(), self.display());
        unicode
    }
}
