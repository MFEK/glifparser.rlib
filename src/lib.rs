#![feature(trait_alias)]

pub mod error;

mod anchor;
mod codepoint; // Unicode codepoint formatter
mod component;
pub mod glif;
mod outline;
mod point;

pub use anchor::Anchor;
pub use crate::codepoint::Codepoint;
pub use crate::component::{GlifComponent, Component};
pub use crate::glif::{Glif, read, write};
pub use crate::outline::{Contour, Outline};
pub use crate::point::{Point, Handle, WhichHandle};
