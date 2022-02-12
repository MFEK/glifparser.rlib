//! .glif `<image>` w/ability to read to a bitmap if filename valid

mod xml;

use std::io;
use std::fs;
use std::path;
use std::rc::Rc;

use crate::color::Color;
use crate::error::GlifParserError;
use crate::glif::GlifLike;
use crate::matrix::GlifMatrix;

use serde::{Serialize, Deserialize};
use integer_or_float::IntegerOrFloat;
use kurbo::Affine;
use log::warn;
use image::{self, DynamicImage, io::Reader};

#[derive(Debug, Clone, PartialEq)]
pub enum DataLoadState {
    /// Image loading hasn't even been attempted yet
    NotTried,
    /// Image loading tried, but failed to read from disk
    TriedAndFailed,
    /// Image loaded from disk to data, but not yet decoded
    Loaded,
    /// Image loaded, but decoding it to a bitmap failed
    LoadedDecodeFailed,
    /// Image has been loaded and decoded to a bitmap
    Decoded,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataOrBitmap {
    Data(Vec<u8>),
    /// `pixels` always RGBA8888
    Bitmap{pixels: Vec<u8>, width: u32, height: u32}
}

impl DataOrBitmap {
    fn unwrap_data(&self) -> &Vec<u8> {
        match self {
            DataOrBitmap::Data(v) => &v,
            DataOrBitmap::Bitmap{..} => panic!("Unwrapped data of bitmap variant")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageData {
    pub data: DataOrBitmap,
    pub state: DataLoadState
}

impl ImageData {
    fn new() -> Self {
        Self {
            data: DataOrBitmap::Data(vec![]),
            state: DataLoadState::NotTried
        }
    }

    pub fn guess_codec(&self) -> ImageCodec {
        let codec = match self.data.unwrap_data().chunks(12).next() {
            Some(&[0x42, 0x4D, _, _, _, _, _, _, _, _, _, _]) => ImageCodec::BMP,
            Some(&[0xFF, 0xD8, _, _, _, _, _, _, _, _, _, _]) => ImageCodec::JPEG,
            Some(&[0x89, 0x50, 0x4E, 0x47, _, _, _, _, _, _, _, _]) => ImageCodec::PNG,
            Some(&[0x47, 0x49, 0x46, 0x38, _, _, _, _, _, _, _, _]) => ImageCodec::GIF,
            Some(&[0x49, 0x49, 0x2A, 0x00, _, _, _, _, _, _, _, _]) => ImageCodec::TIFF,
            Some(&[0x4D, 0x4D, 0x00, 0x2A, _, _, _, _, _, _, _, _]) => ImageCodec::TIFF,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlifImage {
    pub filename: path::PathBuf,
    pub xScale: IntegerOrFloat,
    pub xyScale: IntegerOrFloat,
    pub yxScale: IntegerOrFloat,
    pub yScale: IntegerOrFloat,
    pub xOffset: IntegerOrFloat,
    pub yOffset: IntegerOrFloat,
    pub identifier: Option<String>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub filename: path::PathBuf,
    pub data: ImageData,
    pub codec: ImageCodec,
    pub matrix: Affine,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageCodec {
    Unknown,
    PNG,
    JPEG,
    TIFF,
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
            identifier: None,
            color: None,
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

    pub fn set_matrix(&mut self, matrix: impl Into<Affine>) {
        let coeffs = matrix.into().as_coeffs();
        self.xScale = coeffs[0].into();
        self.xyScale = coeffs[1].into();
        self.yxScale = coeffs[2].into();
        self.yScale = coeffs[3].into();
        self.xOffset = coeffs[4].into();
        self.yOffset = coeffs[5].into();
    }
}

impl GlifImage {
    pub fn to_image_of(&self, glif: &dyn GlifLike) -> Result<Image, GlifParserError> {
        let mut ret = Image::new();
        let mut filename = glif.filename().as_ref().ok_or(GlifParserError::GlifFilenameNotSet(glif.name().clone()))?.clone().parent().ok_or(GlifParserError::GlifFilenameInsane("Failed to parent".to_string()))?.to_path_buf();
        filename.push("..");
        filename.push("images");
        filename.push(self.filename.clone());
        ret.filename = filename;
        ret.load()?;
        ret.matrix = self.matrix().into();
        ret.color = self.color;
        Ok(ret)
    }
}

impl Image {
    pub fn from_filename<P: Into<path::PathBuf>>(p: P) -> Result<Self, GlifParserError> {
        let mut ret = Self::new();
        ret.filename = p.into();
        ret.load()?;
        Ok(ret)
    }

    fn new() -> Self {
        Self {
            filename: path::PathBuf::new(),
            data: ImageData::new(),
            codec: ImageCodec::Unknown,
            matrix: Affine::IDENTITY,
            color: None,
        }
    }

    pub fn load(&mut self) -> Result<(), GlifParserError> {
        self.data.data = DataOrBitmap::Data(fs::read(&self.filename).or_else(|e| {
            self.data.state = DataLoadState::TriedAndFailed;
            Err(GlifParserError::ImageIoError(Some(Rc::new(e))))
        })?);
        self.codec = self.data.guess_codec();
        self.data.state = DataLoadState::Loaded;
        Ok(())
    }

    pub fn data(self) -> Result<Vec<u8>, GlifParserError> {
        match self.data.state {
            DataLoadState::NotTried => Err(GlifParserError::ImageNotLoaded),
            DataLoadState::TriedAndFailed => Err(GlifParserError::ImageIoError(None)),
            DataLoadState::Loaded => Ok(self.data.data.unwrap_data().clone()),
            _ => unimplemented!()
        }
    }

    /// bitmap is guaranteed to always be in RGBA8888 format. Meaning, for each pixel, there's a
    /// [u8; 4]. So a 3x1 image may look like [3, 3, 3, 255, 3, 3, 3, 255, 3, 3, 3, 255].to_vec().
    pub fn decode(&mut self) -> Result<(), GlifParserError> {
        let raw_data = match &self.data.data {
            DataOrBitmap::Data(d) => d,
            DataOrBitmap::Bitmap { .. } => Err(GlifParserError::ImageIoError(None))?
        };
        let reader = Reader::new(io::Cursor::new(raw_data)).with_guessed_format().or_else(|e|Err(GlifParserError::ImageIoError(Some(Rc::new(e)))))?;
        #[cfg(not(feature = "more-image-formats"))]
        if !(reader.format() == Some(image::ImageFormat::Png)) {
            self.data.state = DataLoadState::LoadedDecodeFailed;
            Err(GlifParserError::ImageNotPNG)?
        }
        let bitmap = reader.decode().or_else(|_| {
            self.data.state = DataLoadState::LoadedDecodeFailed;
            Err(GlifParserError::ImageNotDecodable)
        })?;

        let mut pixels;
        if let Some(color) = self.color {
            pixels = DynamicImage::ImageLuma8(bitmap.to_luma8()).to_rgba8();
            for pixel in pixels.chunks_mut(4) {
                pixel[0] = (pixel[0] * color.r).into();
                pixel[1] = (pixel[1] * color.g).into();
                pixel[2] = (pixel[2] * color.b).into();
                pixel[3] = (pixel[3] * color.a).into();
            }
        } else {
            pixels = bitmap.to_rgba8();
        }

        self.data.data = DataOrBitmap::Bitmap { pixels: pixels.to_vec(), width: pixels.width(), height: pixels.height() };
        self.data.state = DataLoadState::Decoded;
        Ok(())
    }
}
