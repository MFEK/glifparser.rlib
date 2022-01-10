#![cfg(feature = "skia")]

use test_log::test;

use glifparser::outline::{FromKurbo as _, IntoKurbo as _, Outline};
use glifparser::Glif;
use glifparser;
use kurbo::BezPath;
use kurbo::Shape as _;

#[test]
fn test_kurbo_from() {
    let mut path: BezPath = kurbo::Circle{center:kurbo::Point::new(0., 0.), radius:50.0}.to_path(1.0);
    path.close_path();
    path.extend(kurbo::Rect::new(0., 0., 100., 100.).to_path(1.0));
    path.close_path();
    let mut path2 = BezPath::new();
    path2.move_to((30., 30.));
    path2.curve_to((-35., 80.), (-150., -50.), (-100., -100.));
    path.extend(&path2);
    let outline = Outline::<()>::from_kurbo(&path);
    eprintln!("{:?}", &outline);
    let kpath = outline.clone().into_kurbo().unwrap();
    let outline2 = Outline::<()>::from_kurbo(&kpath);
    let mut glif = Glif::new();
    eprintln!("{:?}", &kpath);
    eprintln!("{:?}", &outline2);
    glif.outline = Some(outline);
    println!("{}", glifparser::write(&glif).unwrap());
    glif.outline = Some(outline2);
    //println!("{}", glifparser::write(&glif).unwrap());
}
