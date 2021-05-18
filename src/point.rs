use std::fmt::Debug;
use serde::{Serialize, Deserialize};
/// A "close to the source" .glif `<point>`
#[derive(Clone, Debug, PartialEq)]
pub struct GlifPoint {
    pub x: f32,
    pub y: f32,
    pub smooth: bool,
    pub name: Option<String>,
    pub ptype: PointType,
}

impl GlifPoint {
    pub fn new() -> GlifPoint {
        GlifPoint {
            x: 0.,
            y: 0.,
            ptype: PointType::Undefined,
            smooth: false,
            name: None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointType {
    Undefined,
    Move,
    Curve,
    QCurve,
    QClose,
    Line,
    OffCurve,
} // Undefined used by new(), shouldn't appear in Point<PointData> structs

/// A handle on a point
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Handle {
    Colocated,
    At(f32, f32),
}

impl From<Option<&GlifPoint>> for Handle {
    fn from(point: Option<&GlifPoint>) -> Handle {
        match point {
            Some(p) => Handle::At(p.x, p.y),
            None => Handle::Colocated,
        }
    }
}

// The nightly feature is superior because it means people don't need to write e.g.
// `impl PointData for TheirPointDataType {}`
/// API consumers may put any clonable type as an associated type to Glif, which will appear along
/// with each Point. You could use this to implement, e.g., hyperbeziers. The Glif Point's would
/// still represent a Bézier curve, but you could put hyperbezier info along with the Point.
pub trait PointData = Clone + Debug + Serialize;

/// A Skia-friendly point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point<PD> {
    pub x: f32,
    pub y: f32,
    pub a: Handle,
    pub b: Handle,
    pub name: Option<String>,
    pub ptype: PointType,
    pub data: Option<PD>,
}

/// For use by ``Point::handle_or_colocated``
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum WhichHandle {
    Neither,
    A,
    B,
}

impl<PD: PointData> Point<PD> {
    pub fn new() -> Point<PD> {
        Point {
            x: 0.,
            y: 0.,
            a: Handle::Colocated,
            b: Handle::Colocated,
            ptype: PointType::Undefined,
            name: None,
            data: None,
        }
    }

    /// Make a point from its x and y position and type
    pub fn from_x_y_type(at: (f32, f32), ptype: PointType) -> Point<PD> {
        Point {
            x: at.0,
            y: at.1,
            a: Handle::Colocated,
            b: Handle::Colocated,
            ptype: ptype,
            name: None,
            data: None,
        }
    }

    /// Return an x, y position for a point, or one of its handles. If called with
    /// WhichHandle::Neither, return position for point.
    pub fn handle_or_colocated(
        &self,
        which: WhichHandle,
        transform_x: fn(f32) -> f32,
        transform_y: fn(f32) -> f32,
    ) -> (f32, f32) {
        let handle = match which {
            WhichHandle::A => self.a,
            WhichHandle::B => self.b,
            WhichHandle::Neither => Handle::Colocated,
        };
        match handle {
            Handle::At(x, y) => (transform_x(x), transform_y(y)),
            Handle::Colocated => (transform_x(self.x), transform_y(self.y)),
        }
    }
}

pub fn parse_point_type(pt: Option<&str>) -> PointType {
    match pt {
        Some("move") => PointType::Move,
        Some("line") => PointType::Line,
        Some("qcurve") => PointType::QCurve,
        Some("curve") => PointType::Curve,
        _ => PointType::OffCurve,
    }
}

pub fn point_type_to_string(ptype: PointType) -> Option<String>
{
    return match ptype{
        PointType::Undefined => None,
        PointType::OffCurve => None,
        PointType::QClose => None, // should probably be removed from PointType
        PointType::Move => Some(String::from("move")),
        PointType::Curve => Some(String::from("curve")),
        PointType::QCurve => Some(String::from("qcurve")),
        PointType::Line => Some(String::from("line")),
    }
}
