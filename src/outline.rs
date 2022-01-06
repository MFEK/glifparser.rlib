pub mod contour;
pub mod create;
mod kurbo;
pub use self::kurbo::*;
mod refigure;
pub use refigure::*;
pub mod skia;

use log;
#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};

use crate::point::{GlifPoint, Point, PointType};

pub type Contour<PD> = Vec<Point<PD>>;
pub type Outline<PD> = Vec<Contour<PD>>;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutlineType {
    Cubic,
    Quadratic,
    // As yet unimplemented.
    // Will be in <lib> with cubic Bezier equivalents in <outline>.
    Spiro,
}

impl Default for OutlineType {
    fn default() -> OutlineType {
        OutlineType::Cubic
    }
}

pub type GlifContour = Vec<GlifPoint>;
pub type GlifOutline = Vec<GlifContour>;

pub fn get_outline_type(goutline: &GlifOutline) -> OutlineType {
    for gc in goutline.iter() {
        for gp in gc.iter() {
            match gp.ptype {
                PointType::Curve => return OutlineType::Cubic,
                PointType::QCurve => return OutlineType::Quadratic,
                _ => {}
            }
        }
    }
    log::debug!("Defaulting outline with only lines or unrecognized points to cubic");
    OutlineType::Cubic // path has no off-curve point, only lines
}
