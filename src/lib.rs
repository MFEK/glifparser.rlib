pub mod error;

mod anchor;
mod codepoint; // Unicode codepoint formatter
mod color; // glif RGBA color for guidelines etc
pub mod matrix;
mod component;
pub mod glif;
mod guideline;
#[cfg(feature = "glifimage")]
pub mod image;
pub mod outline;
pub mod point;

pub use crate::anchor::Anchor;
pub use crate::codepoint::Codepoint;
pub use crate::color::Color;
pub use crate::component::{FlattenedGlif, GlifComponent, Component, ComponentRect};
pub use crate::glif::{read, read_from_filename, write, write_to_filename};
pub use crate::glif::Glif;
pub use crate::glif::xml;
#[cfg(feature = "mfek")]
pub use crate::glif::mfek::{MFEKGlif, VWSContour, JoinType, CapType};
pub use crate::guideline::{Guideline, GuidelinePoint};
#[cfg(feature = "glifimage")]
pub use crate::image::{GlifImage, Image, ImageCodec};
pub use crate::outline::{contour, Contour, Outline, OutlineType};
pub use crate::point::{Point, PointType, Handle, WhichHandle, PointData};

pub use integer_or_float::IntegerOrFloat;
pub use trees::{Forest, Tree};
