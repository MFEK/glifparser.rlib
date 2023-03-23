use super::IsValid;

#[cfg(feature = "glifserde")]
use serde::Serialize;

use std::fmt::Debug;

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

