pub mod create;

use log::info;

use crate::point::{GlifPoint, Point, PointType};

pub type Contour<PD> = Vec<Point<PD>>;
pub type Outline<PD> = Vec<Contour<PD>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutlineType {
    Cubic,
    Quadratic,
    // As yet unimplemented.
    // Will be in <lib> with cubic Bezier equivalents in <outline>.
    Spiro,
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
    info!("Defaulting outline with only lines or unrecognized points to cubic");
    OutlineType::Cubic // path has no off-curve point, only lines
}
