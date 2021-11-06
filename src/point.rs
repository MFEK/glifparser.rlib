pub mod conv;

use std::fmt::{Display, Debug};
use std::str::FromStr;
#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};
/// A "close to the source" .glif `<point>`
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlifPoint {
    pub x: f32,
    pub y: f32,
    pub smooth: bool,
    pub name: Option<String>,
    pub ptype: PointType,
}

impl GlifPoint {
    pub fn new() -> GlifPoint {
        Self::default()
    }
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointType {
    Undefined,
    /// .glif "move", can act as any point type!
    Move,
    /// .glif "curve" (cubic Bézier point to be followed by two off-curve points)
    Curve,
    /// .glif "qcurve" (quadratic Bézier point to be followed by one…*ish* [see spec] off-curve points)
    QCurve,
    /// TODO: Remove. DEPRECATED
    QClose,
    /// .glif "line"
    Line,
    /// .glif "offcurve" or ""
    OffCurve,
} // Undefined used by new(), shouldn't appear in Point<PointData> structs

/// A handle on a point
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
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
#[cfg(feature = "glifserde")]
pub trait PointData = Clone + Default + Debug + Serialize;
#[cfg(not(feature = "glifserde"))]
pub trait PointData = Clone + Default + Debug;

/// A Skia-friendly point
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point<PD: PointData> {
    pub x: f32,
    pub y: f32,
    pub a: Handle,
    pub b: Handle,
    pub name: Option<String>,
    pub ptype: PointType,
    pub data: Option<PD>,
}

/// For use by ``Point::handle_or_colocated``
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WhichHandle {
    Neither,
    A,
    B,
}

impl<PD: PointData> Point<PD> {
    pub fn new() -> Point<PD> {
        Self::default()
    }

    /// Make a point from its x and y position and type
    pub fn from_x_y_type((x, y): (f32, f32), ptype: PointType) -> Point<PD> {
        Point { x, y, ptype, ..Default::default() }
    }

    /// Make a point from its x and y position, handles and type
    pub fn from_x_y_a_b_type((x, y): (f32, f32), (a, b): (Handle, Handle), ptype: PointType) -> Point<PD> {
        Point { x, y, a, b, ptype, ..Default::default() }
    }

    /// Make a point from its x and y position, handles and type
    pub fn from_fields((x, y): (f32, f32), (a, b): (Handle, Handle), ptype: PointType, name: Option<String>, data: Option<PD>) -> Point<PD> {
        Point { x, y, a, b, ptype, name, data }
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

impl Default for Handle {
    fn default() -> Self { Handle::Colocated }
}

impl Default for PointType {
    fn default() -> PointType { PointType::Undefined }
}

impl Default for WhichHandle {
    fn default() -> Self { WhichHandle::Neither }
}


impl FromStr for PointType {
    type Err = ();
    fn from_str(s: &str) -> Result<PointType, ()> {
        Ok(match s {
            "move" => PointType::Move,
            "line" => PointType::Line,
            "qcurve" => PointType::QCurve,
            "curve" => PointType::Curve,
            _ => PointType::OffCurve,
        })
    }
}

impl FromStr for WhichHandle {
    type Err = ();
    fn from_str(s: &str) -> Result<WhichHandle, ()> {
        Ok(match s.trim() {
            "A" | "a" => WhichHandle::A,
            "B" | "b" => WhichHandle::B,
            _ => WhichHandle::Neither,
        })
    }
}

impl From<&str> for PointType {
    fn from(s: &str) -> Self {
        PointType::from_str(s).unwrap()
    }
}

impl From<&str> for WhichHandle {
    fn from(s: &str) -> Self {
        WhichHandle::from_str(s).unwrap()
    }
}

impl Display for PointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            PointType::Undefined => "undefined",
            PointType::OffCurve => "offcurve",
            PointType::QClose => "qclose",
            PointType::Move => "move",
            PointType::Curve => "curve",
            PointType::QCurve => "qcurve",
            PointType::Line => "line",
        })
    }
}

impl PointType {
    pub fn should_write_to_ufo(&self) -> bool {
        match self {
            PointType::Undefined | PointType::QClose => false,
            _ => true,
        }
    }
}
