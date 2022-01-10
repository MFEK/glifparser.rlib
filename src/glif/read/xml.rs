use crate::error::GlifParserResult;

pub trait FromXML: super::super::write::IntoXML + Sized {
    fn from_xml(xml: &[u8]) -> GlifParserResult<Self>;
    fn from_xml_string(xml: &String) -> GlifParserResult<Self> {
        Self::from_xml(xml.as_bytes())
    }
}
