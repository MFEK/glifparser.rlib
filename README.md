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
first software this was written for, is a detached UFO .glif editor.
