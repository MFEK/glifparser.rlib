# `glifparser.rs`

A parser and writer for UFO `.glif` files.

The primarily difference between this and Norad is that this returns
Skia-friendly points. A cubic B&eacute;zier spline has two handles, `a` and
`b`, and `glifparser` parses the list of on- and off-curve points to come up
with this. (A quadratic B&eacute;zier spline uses the same `Point` type but
will always have handle `b` set to `Handle::Colocated`.)

Another difference is that it supports glyph components more fully, and allows
you to flatten the components to another Glif representing the outlines of all
the components, plus the outlines in the original glyph.

Yet a third difference is the support for private libraries, stored in
comments. This is for support of the `<MFEK>` comment.

Since this library considers .glif files as detached from .ufo files, its
approach is much different from Norad's as well. This is because MFEKglif, the
first software this was written for, is both a _detached_ and an _attached_ UFO 
.glif editor (and viewer). `glifparser` can do more if it knows the .glif is in
a UFO. For example, the aforementioned components feature. It can also read the
image data if it knows that the .glif is part of a parent UFO. However, it is
designed from the beginning to work in both modes: you have to upgrade the 
`GlifImage` it provides you to a regular `Image` which will contain the data if
it indeed exists in the parent UFO; same for `GlifComponent` vs. `Component`.
