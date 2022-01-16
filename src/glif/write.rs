mod xml;
pub use xml::{IntoXML, TryIntoXML};
use xml::*;

use super::{Glif, Lib};
use crate::codepoint::Codepoint;
use crate::error::GlifParserError;
use crate::point::PointData;

use std::fs;
use std::mem;
use std::path::Path;

pub fn write_ufo_glif_to_filename<F, PD>(glif: &Glif<PD>, filename: F) -> Result<(), GlifParserError>
where
    F: AsRef<Path> + Clone,
    PD: PointData,
{
    let glifxml: String = write_ufo_glif(glif)?;
    fs::write(filename, glifxml).or(Err(GlifParserError::XmlWriteError(
        "Failed to write to filename".to_string(),
    )))
}

pub fn write_ufo_glif<PD: PointData>(glif: &Glif<PD>) -> Result<String, GlifParserError> {
    let ret = write_ufo_glif_data(glif)?;
    Ok(String::from_utf8(ret)?)
}

/// Write Glif struct to UFO .glif XML
pub fn write_ufo_glif_data<PD: PointData>(glif: &Glif<PD>) -> Result<Vec<u8>, GlifParserError> {
    let config = xmltree::EmitterConfig::new()
        .perform_indent(true)
        .pad_self_closing(false)
        .autopad_comments(false);

    let glyph = glif.xml();
    let mut ret_string: Vec<u8> = Vec::with_capacity(mem::size_of_val(&glyph)); // size_of_val is an estimate!
    glyph.write_with_config(&mut ret_string, config)?;

    Ok(ret_string)
}

impl<PD: PointData> IntoXML for Glif<PD> {
    fn xml(&self) -> Element {
        let mut glyph = Element::new("glyph");
        glyph.attributes.insert("name".to_owned(), self.name.to_string());
        glyph.attributes.insert("format".to_owned(), "2".to_string());

        if self.width.is_some() {
            glyph.children.push(XMLNode::Element(self.advance_xml()));
        };

        for hex in self.unicode.iter().map(|u| u as &dyn Codepoint) {
            glyph.children.push(XMLNode::Element(hex.xml()));
        }

        for anchor in self.anchors.iter() {
            glyph.children.push(XMLNode::Element(anchor.xml()));
        }

        let mut outline_node = self.outline.as_ref().map(|o|o.xml()).unwrap_or(Element::new("outline"));

        for component in &self.components.vec {
            outline_node.children.push(XMLNode::Element(component.xml()));
        }

        if outline_node.children.len() >= 1 {
            glyph.children.push(XMLNode::Element(outline_node));
        }

        #[cfg(feature = "glifimage")]
        for image in &self.images {
            if let Ok(el) = image.try_xml() {
                glyph.children.push(XMLNode::Element(el));
            }
        }

        for guideline in &self.guidelines {
            glyph.children.push(XMLNode::Element(guideline.xml()));
        }

        if let Some(note) = self.note.as_ref().map(|n| n.clone()) {
            glyph.children.push(XMLNode::Element(Element {
                children: vec![xml::Text(note)],
                ..Element::new("note")
            }));
        }

        #[rustfmt::skip]
        let mut lib = match &self.lib {
            Lib::Plist(lib_node) => {
                let mut plist_buf: Vec<u8> = vec![];
                match plist::to_writer_xml(&mut plist_buf, &lib_node).map(|()|Element::parse(plist_buf.as_slice())) {
                    Ok(Ok(plib)) => (plib),
                    Ok(Err(e)) => {
                        log::error!("Failed to write .glif <lib> as an inline plist. xmltree error: {:?}.", e);
                        return glyph
                    }
                    Err(e) => {
                        log::error!("Failed to write .glif <lib> as an inline plist. plist.rlib error: {:?}.", e);
                        return glyph
                    }
                }
            }
            Lib::Xml(lib) => lib.clone(),
            Lib::None => return glyph,
        };

        // plist library will transform the root to <plist>
        lib.name = String::from("lib");
        // and will add a version, which makes little sense inline, and is likely invalid in .glif
        // (?)
        lib.attributes.clear();

        glyph.children.push(XMLNode::Element(lib));

        glyph
    }
}
