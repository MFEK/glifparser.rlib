//! A parser and writer for UFO `.glif` files.
//!
//! © 2020–2022 Fredrick R. Brennan and MFEK Authors
//!
//! `glifparser` supports the entire `.glif` spec as of 12 April 2021.
//!
//! `glifparser` is not `norad` and is not meant to implement the UFO spec.

// This #[macro_use] is necessary even in Rust 2021 because it auto-imports derive macros.
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate derivative;

pub mod error;

pub mod anchor;
mod codepoint; // Unicode codepoint formatter
pub mod color; // glif RGBA color for guidelines etc
pub mod matrix;
pub mod component;
pub mod glif;
pub mod guideline;
#[cfg(feature = "glifimage")]
pub mod image;
pub mod outline;
pub mod pedantry;
pub mod point;
pub mod string;

pub use crate::anchor::Anchor;
pub use crate::codepoint::Codepoint;
pub use crate::color::Color;
pub use crate::component::{FlattenedGlif, GlifComponent, Component, ComponentRect};
pub use crate::glif::{read, read_pedantic, read_from_filename, read_from_filename_pedantic, write, write_to_filename};
pub use crate::glif::Glif;
pub use crate::glif::xml;
#[cfg(feature = "mfek")]
pub use crate::glif::mfek::{MFEKGlif, VWSContour, JoinType, CapType};
pub use crate::guideline::{Guideline, GuidelinePoint};
#[cfg(feature = "glifimage")]
pub use crate::image::{GlifImage, Image, ImageCodec};
pub use crate::outline::{contour, Contour, Outline, OutlineType};
pub use crate::pedantry::Pedantry;
pub use crate::point::{IsValid, PointLike};
pub use crate::point::{Point, PointType, Handle, WhichHandle, PointData};
pub use crate::string::GlifString;

pub use integer_or_float::IntegerOrFloat;
