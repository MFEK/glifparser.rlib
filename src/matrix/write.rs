#[rustfmt::skip]
// Both components and images have the same matrix/identifier values.
macro_rules! write_matrix {
    ($xml_el:ident, $struct:ident) => {
        match $struct.identifier {
            Some(ref id) => {$xml_el.attributes.insert("identifier".to_string(), id.clone());},
            None  => {}
        }
        $xml_el.attributes.insert("xScale".to_string(), $struct.xScale.to_string());
        $xml_el.attributes.insert("xyScale".to_string(), $struct.xyScale.to_string());
        $xml_el.attributes.insert("yxScale".to_string(), $struct.yxScale.to_string());
        $xml_el.attributes.insert("yScale".to_string(), $struct.yScale.to_string());
        $xml_el.attributes.insert("xOffset".to_string(), $struct.xOffset.to_string());
        $xml_el.attributes.insert("yOffset".to_string(), $struct.yOffset.to_string());
    }
}

pub(crate) use write_matrix;
