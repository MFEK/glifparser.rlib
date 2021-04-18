#![feature(trait_alias)]

pub mod error;

mod anchor;
mod codepoint; // Unicode codepoint formatter
mod matrix;
mod component;
pub mod glif;
mod outline;
mod point;
mod image;

pub use anchor::Anchor;
pub use crate::codepoint::Codepoint;
pub use crate::component::{GlifComponent, Component};
pub use crate::glif::{Glif, read, write};
pub use crate::glif::mfek::{MFEKGlif, VWSContour, JoinType, CapType};
pub use crate::image::{GlifImage, Image, ImageCodec};
pub use crate::outline::{Contour, Outline, OutlineType};
pub use crate::point::{Point, PointType, Handle, WhichHandle, PointData};
