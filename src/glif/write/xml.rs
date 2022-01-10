pub(crate) use crate::xml::*;

use crate::error::GlifParserResult;

pub trait IntoXML {
    fn xml(&self) -> Element;
    fn into_xml(self) -> Element
    where
        Self: Sized,
    {
        (&self).xml()
    }
}

pub trait TryIntoXML {
    fn try_xml(&self) -> GlifParserResult<Element>;
}
