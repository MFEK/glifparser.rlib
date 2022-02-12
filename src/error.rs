//! Provides main error type [`GlifParserError`] & its impl's

#[cfg(feature = "mfek")]
pub mod mfek;

mod string;
pub use string::*;

use crate::point::PointType;

use std::error::Error;
use std::fmt::{Formatter, Display};
use std::io;
use std::rc::Rc;

use xmltree::{ParseError, Error as XMLTreeError};
#[cfg(feature = "glifserde")]
use plist::Error as PlistError;

pub type GlifParserResult<T> = Result<T, GlifParserError>;

#[derive(Debug, Clone)]
pub enum GlifParserError {
    /// OS error when reading glif
    GlifFileIoError(Option<Rc<io::Error>>),
    /// Self-built Outline/Contour error.
    GlifOutlineHasBadPointType{ci: usize, pi: usize, ptype: PointType},
    GlifContourHasBadPointType{pi: usize, ptype: PointType},

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
    /// The XML making up the glif is invalid
    PedanticXmlParseError(String),
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
    TypeConversionError{req_type: &'static str, req_variant: String},

    /// A requested point index is out of bounds
    ContourLenOneUnexpected,
    ContourLenZeroUnexpected,
    PointIdxOutOfBounds{idx: usize, len: usize},
    /// No previous on an open contour.
    // usize value = contour length for these 2
    ContourNoPrevious(usize),
    /// No next on an open contour
    ContourNoNext(usize),
}

impl Display for GlifParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "glifparser error: {}", match self {
            Self::GlifFileIoError(ioe) => {
                format!("System error when loading glif file: {:?}", ioe)
            },
            Self::GlifOutlineHasBadPointType{ci, pi, ptype} => {
                format!("Bad point type {:?} in outline @({}, {}))", ptype, ci, pi)
            },
            Self::GlifContourHasBadPointType{pi, ptype} => {
                format!("Bad point type {:?} in contour @{})", ptype, pi)
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
            Self::PedanticXmlParseError(s) => {
                format!("XML error (requested pedantry, would not normally be an error): {}", &s)
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

            Self::TypeConversionError { req_type, req_variant } => {
                format!("Type conversion error: {} not in {}", req_variant, req_type)
            }

            Self::PointIdxOutOfBounds { idx, len } => {
                format!("The point index {} is out of bounds as self.len() == {}", idx, len)
            }
            Self::ContourLenOneUnexpected => {
                format!("On a contour of length one, there's no previous/next point")
            }
            Self::ContourLenZeroUnexpected => {
                format!("On an empty invalid contour (len == 0), there's no previous/next point")
            }

            Self::ContourNoPrevious(len) => {
                format!("Asked for previous index of 0 on an open contour (len {})", len)
            }

            Self::ContourNoNext(len) => {
                format!("Asked for next index of last point, {}, on an open contour", len)
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
#[cfg(feature = "glifserde")]
impl From<PlistError> for GlifParserError {
    fn from(_e: PlistError) -> Self {
        GlifParserError::GlifLibError
    }
}

impl From<std::string::FromUtf8Error> for GlifParserError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self::GlifNotUtf8
    }
}

impl Error for GlifParserError {}
