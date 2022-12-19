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

use log;
#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};

use crate::component::GlifComponent;
use crate::point::{GlifPoint, Point, PointType};

pub type Contour<PD> = Vec<Point<PD>>;
pub type Outline<PD> = Vec<Contour<PD>>;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GlifOutlineType {
    OnlyCorners = -1,
    Cubic,
    Quadratic,
    Mixed
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OutlineType {
    Cubic,
    Quadratic,
    // As yet unimplemented.
    // Will be in <lib> with cubic Bezier equivalents in <outline>.
    Spiro,
}

impl Default for OutlineType {
    fn default() -> Self {
        Self::Cubic
    }
}

impl Default for GlifOutlineType {
    fn default() -> Self {
        Self::Mixed
    }
}

pub type GlifContour = Vec<GlifPoint>;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GlifOutline {
    pub contours: Vec<GlifContour>,
    pub components: Vec<GlifComponent>,
    pub otype: GlifOutlineType,
    figured_type: bool,
    warned_type: bool,
}

impl From<GlifOutlineType> for OutlineType {
    fn from(got: GlifOutlineType) -> Self {
        match got {
            GlifOutlineType::Mixed => OutlineType::Cubic,
            GlifOutlineType::Cubic => OutlineType::Cubic,
            GlifOutlineType::OnlyCorners => OutlineType::Cubic,
            GlifOutlineType::Quadratic => OutlineType::Quadratic,
        }
    }
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
            otype: GlifOutlineType::default(),
            figured_type: false,
            warned_type: false,
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

impl GlifOutline {
    pub fn get_type(&self) -> GlifOutlineType {
        let (cubics, quadratics): (Vec<_>, Vec<_>) = self.iter().map(|gc|gc.iter().map(|gp|gp.ptype).filter(|pt| *pt == PointType::Curve || *pt == PointType::QCurve)).flatten().partition(|pt| *pt == PointType::Curve);

        match (cubics.len(), quadratics.len()) {
            (0usize, 0usize) => GlifOutlineType::OnlyCorners,
            (0usize, 1usize..) => GlifOutlineType::Quadratic,
            (1usize.., 0usize) => GlifOutlineType::Cubic,
            (_, _) => GlifOutlineType::Mixed, // 1usize..usize::MAX, 1usize..usize::MAX rust-lang/rust/issues/37854
        }
    }

    fn warn_type(&mut self) {
        self.warned_type = true;
        match self.otype {
            GlifOutlineType::Mixed => log::warn!("Outline contains a mix of quadratic and cubic contours! This could lead to all kinds of bugs. What software produced this, anyway…?"),
            GlifOutlineType::OnlyCorners => log::debug!("Outline only has corners; Outline<PD> type will treat it as a cubic."),
            GlifOutlineType::Quadratic => log::warn!("Quadratic curve support in MFEK, while ahead of e.g. Runebender, is still spotty. You are likely to find many bugs, it is recommended you convert to cubic contours."),
            GlifOutlineType::Cubic => log::trace!("Loaded a cubic/corner-only outline (as expected)"),
        }
    }

    pub fn figure_type(&mut self) {
        self.otype = self.get_type();
        self.figured_type = true;
        if !self.warned_type {
            self.warn_type();
            self.warned_type = true;
        }
    }
}
