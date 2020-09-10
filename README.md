# `glifparser.rs`

A parser and writer for UFO `.glif` files.

The primarily difference between this and Norad is that this returns
Skia-friendly points. A cubic B&eacute;zier spline has two handles, `a` and
`b`, and `glifparser` parses the list of on- and off-curve points to come up
with this. (A quadratic B&eacute;zier spline uses the same `Point` type but
will always have handle `b` set to `Handle::Colocated`.)

**NOTE**: This `.glif` parser is at the moment _unversioned_ (0.0.0); it has an
unstable API. Don't use it in your own projects yet; if you insist, tell Cargo
to use a `rev`. Right now it just panics instead of returning a `Result<Glif,
E>` type and has no `Error` enum, which will be fixed in the final
crates.io-friendly API.
