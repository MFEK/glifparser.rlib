#![cfg(feature = "skia")]

use test_log::test;

use glifparser::outline::{FromKurbo as _, IntoKurbo as _, Outline};
use glifparser::outline::skia::*;
use glifparser::Glif;
use glifparser;
use skia_safe::{Path as SkPath, Rect as SkRect};

#[test]
fn test_kurbo_from() {
    let mut path = SkPath::circle((0., 0.), 50.0, None);
    path.add_rect(SkRect::new(0., 0., 100., 100.), None);
    let mut path2 = SkPath::new();
    path2.move_to((30., 30.));
    path2.cubic_to((-35., 80.), (-150., -50.), (-100., -100.));
    path2.move_to((30., 30.));
    path2.close();
    path.add_path(&path2, (0., 0.), None);
    let outline = Outline::<()>::from_skia_path(&path);
    eprintln!("{:?}", &outline);
    let kpath = outline.into_kurbo().unwrap();
    let outline2 = Outline::<()>::from_kurbo(&kpath);
    let mut glif = Glif::new();
    eprintln!("{:?}", &kpath);
    eprintln!("{:?}", &outline2);
    glif.outline = Some(outline2);
    eprintln!("{}", glifparser::write(&glif).unwrap());
}
