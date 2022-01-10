use crate::error::{GlifParserError, GlifParserResult};
use crate::matrix;
use crate::xml::{Element, TryIntoXML};

use super::GlifImage;

impl TryIntoXML for GlifImage {
    fn try_xml(&self) -> GlifParserResult<Element> {
        let mut image_node = Element::new("image");
        let image_fn = self.filename.to_string_lossy().into_owned();
        if self.filename.to_str().map(|s: &str| s.len()).unwrap_or(0) != image_fn.as_str().len() {
            log::error!("image filename `{:?}` not UTF8, skipping!", self.filename);
            return Err(GlifParserError::GlifNotUtf8)
        } else {
            image_node.attributes.insert(
                "fileName".to_string(),
                self.filename.to_str().unwrap().to_string(),
            );
        }
        matrix::write!(image_node, self);
        Ok(image_node)
    }
}
