use crate::xml;

#[derive(Clone, Debug, PartialEq)]
pub enum Lib {
    None,
    Plist(plist::Dictionary),
    /// This variant is highly undesirable to see as output and means that the user's glif file has
    /// validity issues. However, to prevent data loss, we attempt to store the broken plist as
    /// XML, as XML is the parent of plist.
    Xml(xml::Element),
}

impl Default for Lib {
    fn default() -> Self {
        Self::None
    }
}
