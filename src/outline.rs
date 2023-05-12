//! .glif `<outline>` and `<contour>`

pub mod contour;
pub use contour::Reverse;
mod conv;
pub use conv::{IntoGlifPoints, ToOutline};
pub use conv::{PenOperations, PenOperationsContour, PenOperationsPath, IntoPenOperations, SplitPenOperations};
pub mod create;
mod kurbo;
pub use self::kurbo::*;
mod quad_to_cubic;
pub use quad_to_cubic::QuadToCubic;
mod refigure;
mod reverse;
pub use refigure::*;
pub mod skia;
mod xml;

#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};

use crate::component::GlifComponent;
use crate::point::{GlifPoint, Point};

pub type Contour<PD> = Vec<Point<PD>>;
pub type Outline<PD> = Vec<Contour<PD>>;

pub type GlifContour = Vec<GlifPoint>;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GlifOutline {
    pub contours: Vec<GlifContour>,
    pub components: Vec<GlifComponent>,
}

impl std::ops::Deref for GlifOutline {
    type Target = Vec<GlifContour>;
    fn deref(&self) -> &Vec<GlifContour> {
        &self.contours
    }
}

impl std::ops::DerefMut for GlifOutline {
    fn deref_mut(&mut self) -> &mut Vec<GlifContour> {
        &mut self.contours
    }
}

impl Default for GlifOutline {
    fn default() -> Self {
        Self {
            contours: vec![],
            components: vec![],
        }
    }
}

impl GlifOutline {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<Vec<GlifContour>> for GlifOutline {
    fn from(v: Vec<GlifContour>) -> Self {
        Self {
            contours: v,
            .. Default::default()
        }
    }
}
