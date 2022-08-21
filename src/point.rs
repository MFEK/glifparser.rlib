//! .glif `<point>`

mod conv;
pub use self::conv::kurbo::*;
mod valid;
pub use self::valid::{IsValid, PointLike};
mod xml;

use std::fmt::{Display, Debug};
use std::str::FromStr;

use integer_or_float::IntegerOrFloat;

#[cfg(feature = "glifserde")]
use serde::{Serialize, Deserialize};

/// A "close to the source" .glif `<point>`
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct GlifPoint {
    pub x: IntegerOrFloat,
    pub y: IntegerOrFloat,
    pub smooth: bool,
    pub name: Option<String>,
    pub ptype: PointType,
}

impl GlifPoint {
    /// Make a point from its x and y position and type
    pub fn from_x_y_type((x, y): (impl Into<IntegerOrFloat>, impl Into<IntegerOrFloat>), ptype: PointType) -> GlifPoint {
        let (x, y) = (x.into(), y.into());
        GlifPoint { x, y, ptype, ..Default::default() }
    }

    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }
}

impl GlifPoint {
    pub fn new() -> GlifPoint {
        Self::default()
    }
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
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
    At(f32, f32),
    Colocated,
}

impl From<Option<&GlifPoint>> for Handle {
    fn from(point: Option<&GlifPoint>) -> Handle {
        match point {
            Some(p) => Handle::At(p.x.into(), p.y.into()),
            None => Handle::Colocated,
        }
    }
}

impl From<Option<(f32, f32)>> for Handle {
    fn from(o: Option<(f32, f32)>) -> Self {
        match o {
            Some((x, y)) => Handle::At(x, y),
            None => Handle::Colocated
        }
    }
}

impl From<(f32, f32)> for Handle {
    fn from((x, y): (f32, f32)) -> Self {
        Handle::At(x, y)
    }
}

impl From<()> for Handle {
    fn from(_: ()) -> Self {
        Handle::Colocated
    }
}

// The nightly feature is superior because it means people don't need to write e.g.
// `impl PointData for TheirPointDataType {}`
/// API consumers may put any clonable type as an associated type to Glif, which will appear along
/// with each Point. You could use this to implement, e.g., hyperbeziers. The Glif Point's would
/// still represent a Bézier curve, but you could put hyperbezier info along with the Point.
///
/// Note that anchors and guidelines receive *the same type*. So, if you wanted to put *different*
/// data along with each, you would need to make an enum like:
///
/// ```rust
/// use glifparser::{Point, PointData};
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
/// pub enum MyPointData {
///     Point(bool),
///     Guideline(u8),
///     Anchor { good: bool },
/// }
/// impl Default for MyPointData {
///     fn default() -> Self {
///         Self::Point(false)
///     }
/// }
/// impl PointData for MyPointData {}
///
/// fn testing() {
///     let mut point = Point::default();
///     point.data = Some(MyPointData::Point(true));
/// }
/// ```
#[cfg(feature = "glifserde")]
pub trait PointData where Self: Clone + Default + Debug + Serialize {}
#[cfg(not(feature = "glifserde"))]
pub trait PointData where Self: Clone + Default + Debug {}
impl PointData for () {}
impl<PD: PointData> IsValid for PD {}

/// A Skia-friendly point
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point<PD: PointData> {
    pub x: f32,
    pub y: f32,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub a: Handle,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub b: Handle,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub name: Option<String>,
    pub ptype: PointType,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub smooth: bool,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub data: Option<PD>,
}

/// For use by ``Point::handle_or_colocated``
/// TODO: Replace with Option<WhichHandle>
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
#[repr(i8)]
pub enum WhichHandle {
    /// TODO: Deprecate Neither.
    Neither = -1,
    A,
    B,
}

impl<PD: PointData> Point<PD> {
    pub fn new() -> Point<PD> {
        Self::default()
    }

    /// Make a point from its x and y position and type
    pub fn from_x_y_type((x, y): (f32, f32), ptype: PointType) -> Point<PD> {
        #[cfg(debug_assertions)] Self::check_ptype(ptype);
        Point { x, y, ptype, ..Default::default() }
    }

    /// Make a point from its x and y position, handles and type
    pub fn from_x_y_a_b_type((x, y): (f32, f32), (a, b): (Handle, Handle), ptype: PointType) -> Point<PD> {
        #[cfg(debug_assertions)] Self::check_ptype(ptype);
        Point { x, y, a, b, ptype, ..Default::default() }
    }

    /// Make a point from its x and y position, handles and type
    pub fn from_fields((x, y): (f32, f32), (a, b): (Handle, Handle), smooth: bool, ptype: PointType, name: Option<String>, data: Option<PD>) -> Point<PD> {
        #[cfg(debug_assertions)] Self::check_ptype(ptype);
        Point { x, y, a, b, smooth, ptype, name, data }
    }

    pub fn handle(&self, which: WhichHandle) -> Handle {
        match which {
            WhichHandle::A => self.a,
            WhichHandle::B => self.b,
            WhichHandle::Neither => {
                log::error!("Used Point::handle(which) to get Neither handle…that shouldn't be valid!");
                Handle::Colocated
            },
        }
    }

    /// Return an x, y position for a point, or one of its handles. If called with
    /// WhichHandle::Neither, return position for point.
    pub fn handle_or_colocated(
        &self,
        which: WhichHandle,
        transform_x: &dyn Fn(f32) -> f32,
        transform_y: &dyn Fn(f32) -> f32,
    ) -> (f32, f32) {
        let handle = self.handle(which);
        match handle {
            Handle::At(x, y) => (transform_x(x), transform_y(y)),
            Handle::Colocated => (transform_x(self.x), transform_y(self.y)),
        }
    }

    pub fn handle_as_gpoint(
        &self,
        which: WhichHandle,
    ) -> GlifPoint {
        let handle = self.handle(which);
        let (x, y) = match handle {
            Handle::At(x, y) => (x, y),
            Handle::Colocated => (self.x, self.y),
        };
        GlifPoint::from_x_y_type((x, y), PointType::OffCurve)
    }

    pub fn handle_as_point(&self, which: WhichHandle) -> Self {
        (&self.handle_as_gpoint(which)).into()
    }

    pub fn handle_as_kpoint(
        &self,
        which: WhichHandle,
    ) -> KurboPoint {
        let p = self.handle_as_gpoint(which);
        KurboPoint::new(f64::from(p.x), f64::from(p.y))
    }

    /// This function is intended for use by generic functions that can work on either handle, to
    /// decrease the use of macros like `move_mirror!(a, b)`.
    pub fn set_handle(&mut self, which: WhichHandle, handle: Handle) {
        match which {
            WhichHandle::A => self.a = handle,
            WhichHandle::B => self.b = handle,
            WhichHandle::Neither => log::error!("Tried to Point::set_handle a WhichHandle::Neither, refusing to set point's x, y"),
        }
    }
}

#[cfg(debug_assertions)]
impl<PD: PointData> Point<PD> {
    fn check_ptype(ptype: PointType) {
        if ptype == PointType::OffCurve {
            panic!("Illegal to create a Point<_> of OffCurve type—only OK for GlifPoint!");
        }
    }
}

impl Default for Handle {
    fn default() -> Handle { Handle::Colocated }
}

impl Default for PointType {
    fn default() -> PointType { PointType::OffCurve }
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
        debug_assert!(s.chars().count() == 1);
        s.trim().chars().nth(0).map(|c| Ok(c.into())).unwrap_or(Err(()))
    }
}

impl From<char> for WhichHandle {
    fn from(c: char) -> WhichHandle {
        match c {
            'A' | 'a' | 'Ａ' | 'ａ' => WhichHandle::A,
            'B' | 'b' | 'Ｂ' | 'ｂ' => WhichHandle::B,
            _ => {
                debug_assert!(c == 0 as char);
                WhichHandle::Neither
            },
        }
    }
}

impl From<Handle> for PointType {
    fn from(h: Handle) -> Self {
        match h {
            Handle::At(..) => Self::Curve,
            Handle::Colocated => Self::Line,
        }
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

impl WhichHandle {
    pub fn opposite(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
            Self::Neither => {
                log::error!("Tried to get opposite handle of Neither, returned Neither. This should not be valid!");
                Self::Neither
            }
        }
    }
}

impl IsValid for WhichHandle {
    fn is_valid(&self) -> bool {
        *self == Self::A || *self == Self::B
    }
}

impl Into<char> for WhichHandle {
    fn into(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::Neither => 0 as char,
        }
    }
}

impl IsValid for PointType {
    fn is_valid(&self) -> bool {
        match *self {
            Self::Move | Self::Line | Self::QCurve | Self::Curve | Self::OffCurve => true,
            _ => false
        }
    }
}

impl PointType {
    pub fn is_valid_oncurve(self) -> bool {
        if self == Self::OffCurve {
            false
        } else {
            self.is_valid()
        }
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
