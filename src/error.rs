use std::fmt::{Formatter, Display};
use std::error::Error;
use std::string;

use xmltree::{ParseError, Error as XMLTreeError};

#[derive(Debug, Clone)]
pub enum GlifParserError {
    /// Glif filename not set
    GlifFilenameNotSet(String),
    /// Glif filename doesn't match name in XML
    GlifFilenameInsane(String),
    /// Components of the glyph form a loop
    GlifComponentsCyclical(String),
    /// Glif isn't UTF8
    GlifNotUtf8(String),
    /// The XML making up the glif is invalid
    XmlParseError(String),
    /// Failures when writing glif XML
    XmlWriteError(String),
    /// The XML is valid, but doesn't meet the UFO .glif spec
    GlifInputError(String),
}

impl Display for GlifParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "glifparser error: {}", match self {
            Self::GlifFilenameNotSet(s) => {
                format!("Glyph filename not set: {}", &s)
            },
            Self::GlifFilenameInsane(s) => {
                format!("Glyph filename not sane: {}", &s)
            },
            Self::GlifNotUtf8(_) => {
                format!("Glyph not utf-8")
            },
            Self::GlifComponentsCyclical(s) => {
                format!("Glyph components are cyclical: {}", &s)
            },
            Self::XmlParseError(s) | Self::XmlWriteError(s) => {
                format!("XML error: {}", &s)
            },
            Self::GlifInputError(s) => {
                format!("Glif format spec error: {}", &s)
            },
        })
    }
}

// the parsing function in read_ufo_glif can only return this error type
impl From<ParseError> for GlifParserError {
    fn from(e: ParseError) -> Self {
        Self::XmlParseError(format!("{}", e))
    }
}

// . . . therefore it's OK to consider this a write-time error type
impl From<XMLTreeError> for GlifParserError {
    fn from(e: XMLTreeError) -> Self {
        Self::XmlWriteError(format!("{}", e))
    }
}

impl From<string::FromUtf8Error> for GlifParserError {
    fn from(_: string::FromUtf8Error) -> Self {
        Self::GlifNotUtf8("".to_string())
    }
}


impl Error for GlifParserError {}
