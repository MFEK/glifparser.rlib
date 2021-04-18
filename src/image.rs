use std::fs;
use std::path;
use std::rc::Rc;

use crate::error::GlifParserError;
use crate::glif::Glif;
use crate::matrix::GlifMatrix;

use integer_or_float::IntegerOrFloat;
use kurbo::Affine;
use log::warn;

#[derive(Debug, Clone, PartialEq)]
pub enum DataLoadState {
    NotTried,
    TriedAndFailed,
    Succeeded
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageData {
    data: Vec<u8>,
    state: DataLoadState
}

impl ImageData {
    fn new() -> Self {
        Self {
            data: vec![],
            state: DataLoadState::NotTried
        }
    }

    pub fn guess_codec(&self) -> ImageCodec {
        let codec = match self.data.chunks(12).next() {
            Some(&[0x42, 0x4D, _, _, _, _, _, _, _, _, _, _]) => ImageCodec::BMP,
            Some(&[0xFF, 0xD8, _, _, _, _, _, _, _, _, _, _]) => ImageCodec::JPEG,
            Some(&[0x89, 0x50, 0x4E, 0x47, _, _, _, _, _, _, _, _]) => ImageCodec::PNG,
            Some(&[0x47, 0x49, 0x46, 0x38, _, _, _, _, _, _, _, _]) => ImageCodec::GIF,
            Some(&[0x52, 0x49, 0x46, 0x46, _, _, _, _, 0x57, 0x45, 0x42, 0x50]) => ImageCodec::WebP,
            _ => ImageCodec::Unknown
        };

        if codec != ImageCodec::PNG {
            warn!("Image not PNG! `fontmake` and other UFO spec conformant progams will refuse to embed the bitmap. MFEKglif may still be able to display it, however, so only use it for proofing, not output.");
        }

        codec
    }
}

#[allow(non_snake_case)] // to match UFO spec https://unifiedfontobject.org/versions/ufo3/glyphs/glif/#image
#[derive(Debug, Clone, PartialEq)]
pub struct GlifImage {
    pub filename: path::PathBuf,
    pub xScale: IntegerOrFloat,
    pub xyScale: IntegerOrFloat,
    pub yxScale: IntegerOrFloat,
    pub yScale: IntegerOrFloat,
    pub xOffset: IntegerOrFloat,
    pub yOffset: IntegerOrFloat,
    pub identifier: Option<String>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub filename: path::PathBuf,
    data: ImageData,
    pub codec: ImageCodec,
    pub matrix: Affine,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageCodec {
    Unknown,
    PNG,
    JPEG,
    GIF,
    WebP,
    BMP,
}

impl GlifImage {
    fn new() -> Self {
        Self {
            filename: path::PathBuf::new(),
            xScale: IntegerOrFloat::Integer(1),
            xyScale: IntegerOrFloat::Integer(0),
            yxScale: IntegerOrFloat::Integer(0),
            yScale: IntegerOrFloat::Integer(1),
            xOffset: IntegerOrFloat::Integer(0),
            yOffset: IntegerOrFloat::Integer(0),
            identifier: None
        }
    }

    pub fn from_filename<P: Into<path::PathBuf>>(p: P) -> Result<Self, GlifParserError> {
        let mut ret = Self::new();
        ret.filename = p.into();
        Ok(ret)
    }
}

impl GlifImage {
    pub fn matrix(&self) -> GlifMatrix {
        GlifMatrix(self.xScale, self.xyScale, self.yxScale, self.yScale, self.xOffset, self.yOffset)
    }
}

use crate::point::PointData;
impl GlifImage {
    pub fn to_image_of<PD: PointData>(&self, glif: &Glif<PD>) -> Result<Image, GlifParserError> {
        let mut ret = Image::new();
        let mut filename = glif.filename.as_ref().ok_or(GlifParserError::GlifFilenameNotSet(glif.name.clone()))?.clone().parent().ok_or(GlifParserError::GlifFilenameInsane("Failed to parent".to_string()))?.to_path_buf();
        filename.push("..");
        filename.push("images");
        filename.push(self.filename.clone());
        ret.filename = filename;
        ret.load()?;
        ret.matrix = self.matrix().into();
        Ok(ret)
    }
}

impl Image {
    /*pub fn from_filename<P: Into<path::PathBuf>>(p: P) -> Result<Self, GlifParserError> {
        let mut ret = Self::new();
        ret.filename = p.into();
        ret.load()?;
        Ok(ret)
    }*/

    fn new() -> Self {
        Self {
            filename: path::PathBuf::new(),
            data: ImageData::new(),
            codec: ImageCodec::Unknown,
            matrix: Affine::IDENTITY
        }
    }

    pub fn load(&mut self) -> Result<(), GlifParserError> {
        self.data.data = fs::read(&self.filename).or_else(|e| {
            self.data.state = DataLoadState::TriedAndFailed;
            Err(GlifParserError::ImageIoError(Some(Rc::new(e))))
        })?;
        self.codec = self.data.guess_codec();
        self.data.state = DataLoadState::Succeeded;
        Ok(())
    }

    pub fn data(self) -> Result<Vec<u8>, GlifParserError> {
        match self.data.state {
            DataLoadState::NotTried => Err(GlifParserError::ImageNotLoaded),
            DataLoadState::TriedAndFailed => Err(GlifParserError::ImageIoError(None)),
            DataLoadState::Succeeded => Ok(self.data.data)
        }
    }
}
