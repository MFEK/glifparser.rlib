use std::error::Error;
use std::fmt::{Formatter, Display};
use std::io;
use std::rc::Rc;
use std::string;

use xmltree::{ParseError, Error as XMLTreeError};
use plist::Error as PlistError;

#[derive(Debug, Clone)]
pub enum GlifParserError {
    /// OS error when reading glif
    GlifFileIoError(Option<Rc<io::Error>>),

    /// Glif filename not set
    GlifFilenameNotSet(String),
    /// Glif filename doesn't match name in XML
    GlifFilenameInsane(String),
    /// Components of the glyph form a loop
    GlifComponentsCyclical(String),
    /// .glif has invalid <lib>
    GlifLibError,

    /// Glif isn't UTF8
    GlifNotUtf8,
    /// The XML making up the glif is invalid
    XmlParseError(String),
    /// Failures when writing glif XML
    XmlWriteError(String),
    /// The XML is valid, but doesn't meet the UFO .glif spec
    GlifInputError(String),

    /// Image not yet read
    ImageNotLoaded,
    /// Image not PNG
    ImageNotPNG,
    /// Image not decodable
    ImageNotDecodable,
    /// OS error when reading image
    ImageIoError(Option<Rc<io::Error>>),

    /// Color (for guidelines, images, etc) not RGBA
    ColorNotRGBA,
    /// Error for use by parse() trait (FromStr)
    TypeConversionError(&'static str, String),
}

impl Display for GlifParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "glifparser error: {}", match self {
            Self::GlifFileIoError(ioe) => {
                format!("System error when loading glif file: {:?}", ioe)
            },
            Self::GlifFilenameNotSet(s) => {
                format!("Glyph filename not set: {}", &s)
            },
            Self::GlifFilenameInsane(s) => {
                format!("Glyph filename not sane: {}", &s)
            },
            Self::GlifNotUtf8 => {
                format!("Glyph not utf-8")
            },
            Self::GlifComponentsCyclical(s) => {
                format!("Glyph components are cyclical: {}", &s)
            },
            Self::GlifLibError => {
                format!("Glif <lib> invalid")
            },

            Self::XmlParseError(s) | Self::XmlWriteError(s) => {
                format!("XML error: {}", &s)
            },
            Self::GlifInputError(s) => {
                format!("Glif format spec error: {}", &s)
            },

            Self::ImageNotLoaded => {
                format!("Tried to access data for image whose data hasn't been loaded")
            },
            Self::ImageNotPNG => {
                format!("Image not formatted as PNG. The glif file format only supports PNG. If you want to support other types, you have to work on the data yourself.")
            },
            Self::ImageNotDecodable => {
                format!("Image not decodable")
            },
            Self::ImageIoError(ioe) => {
                format!("System error when loading image: {:?}", ioe)
            },

            Self::ColorNotRGBA => {
                format!("Color not RGBA")
            },

            Self::TypeConversionError(t, s) => {
                format!("Type conversion error: {} not in {}", s, t)
            }
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
impl From<PlistError> for GlifParserError {
    fn from(_e: PlistError) -> Self {
        GlifParserError::GlifLibError
    }
}

impl From<string::FromUtf8Error> for GlifParserError {
    fn from(_: string::FromUtf8Error) -> Self {
        Self::GlifNotUtf8
    }
}

impl Error for GlifParserError {}
