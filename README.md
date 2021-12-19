# glifparser

A parser and writer for UFO `.glif` files.

`glifparser` is one of the most important MFEK libraries, and is considered a
core library. Almost all modules and non-core libraries are expected to in some
way rely on `glifparser` and its types.

`glifparser` supports the entire `.glif` spec as of 12 April 2021
([`0a79aa7`](https://github.com/MFEK/glifparser.rlib/commit/0a79aa7a050d978b2774f8e32621790e6b5538b2)),
when components were added with full validation support (components may not
include themselves, nor may they include a component which somewhere in its
inclusion tree includes any parent glyph). `glifparser` supports images
completely, including colored images, and can generate the colored to-spec UFO
bitmaps for you.

## Comparison with Norad

First and foremost, `glifparser` has different goals than Norad. `glifparser`
is intended to support _only_ the `.glif` part of the UFO spec, while Norad
intends to support the entire UFO spec. Furthermore, `glifparser` intends to
optionally serialize/deserialize the `<lib/>` elements used by the MFEK project
(when compiled w/`--features=mfek`)—therefore, many non-UFO .glif types can be
found in e.g.
[`src/glif/mfek.rs`](https://github.com/MFEK/glifparser.rlib/blob/master/src/glif/mfek.rs).

The reason this library only implements `.glif` is it considers `.glif` files
as being _possibly detached from `.ufo` files_ (“unparented `.glif`”). At the
time this library was written, Norad did not consider unparented `.glif` files
as being legitimate, but has since added some support:
[`norad::Glyph::load(path: impl
AsRef<Path>)`](https://github.com/linebender/norad/blob/5f0cc9c9b6f923b18c6eddfa481ef9eb9d72335e/src/glyph/mod.rs#L65).
Despite this, however, a lot of Norad's functions won't work as expected on an
unparented `.glif`. `glifparser`, on the other hand, considers all `.glif`'s
unparented until proven otherwise. This means that there are two versions of
all types that rely on other glyph files: a `Glif` prefixed version means that
it is a close representation of the `.glif` XML. For example, `GlifImage`
provides you a close representation of an `<image>` element, while if you
upgrade that to a regular `Image`, (which can fail, if the `.glif` is
unparented), that new type which will contain the data if it indeed exists in
the parent UFO.

The same goes for `GlifComponent` (an unparented `.glif` file referring to a
component that may or may not exist) vs. `Component` (a validated component
retrieved from a `GlifComponent`). `glifparser` can also flatten components (as
in, apply their matrices and insert their points into the parent `.glif`) and
print trees of arbitrary depth representing the base/component relationship.

Another huge difference between `glifparser` and Norad is that `glifparser`
returns Skia-friendly points. Its cubic B&eacute;zier `Point` has two handles,
`a` and `b`, and `glifparser` parses the list of on- and off-curve points to
come up with this. (A quadratic B&eacute;zier spline uses the same `Point` type
but will always have handle `b` set to `Handle::Colocated`.) Norad however does
no contour parsing and just gives you the points to parse yourself.

## Useful traits

There are a lot of useful traits for `glifparser` types that aren't implemented
here, but in other MFEK libraries. For example,
[`MFEKmath::PolarCoordinates`](https://github.com/MFEK/math.rlib/blob/main/src/polar.rs),
implemented on `Point`, allows for getting/setting point handles by polar as
well as Cartesian coordinates (in polar mode, the point is the origin).

You can also find in MFEK/math.rlib piecewise spline types that can be
converted to and from glifparser's `Glif<PD>` and `Outline<PD>` types.

## `PointData`

API consumers may put any clonable type as an associated type to Glif, which
will appear along with each Point. You could use this to implement, e.g.,
hyperbeziers. The Glif Point's would still represent a Bézier curve, but you
could put hyperbezier info along with the Point.

Note that anchors and guidelines receive *the same type*. So, if you wanted to
put *different* data along with each, you would need to make an enum like:

```rust
use glifparser::{Point, PointData};

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MyPointData {
    Point(bool),
    Guideline(u8),
    Anchor { good: bool },
}
impl Default for MyPointData {
    fn default() -> Self {
        Self::Point(false)
    }
}
impl PointData for MyPointData {}

fn testing() {
    let mut point = Point::default();
    point.data = Some(MyPointData::Point(true));
}
```
