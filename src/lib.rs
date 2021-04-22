#![feature(trait_alias)]
#![feature(assoc_char_funcs)]

pub mod error;

mod anchor;
mod codepoint; // Unicode codepoint formatter
mod color; // glif RGBA color for guidelines etc
mod matrix;
mod component;
pub mod glif;
mod guideline;
mod image;
pub mod outline;
mod point;

pub use crate::anchor::Anchor;
pub use crate::codepoint::Codepoint;
pub use crate::color::Color;
pub use crate::component::{GlifComponent, Component, ComponentRect};
pub use crate::glif::{Glif, read, write};
pub use crate::glif::mfek::{MFEKGlif, VWSContour, JoinType, CapType};
pub use crate::guideline::{Guideline, GuidelinePoint};
pub use crate::image::{GlifImage, Image, ImageCodec};
pub use crate::outline::{Contour, Outline, OutlineType};
pub use crate::point::{Point, PointType, Handle, WhichHandle, PointData};
pub use integer_or_float::IntegerOrFloat;

pub use trees::{Forest, Tree};
