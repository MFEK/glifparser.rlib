use test_log;

use glifparser::outline::{FromKurbo as _, IntoKurbo as _, RoundToInt as _, Outline};
use glifparser::Glif;
use glifparser;
use kurbo::BezPath;
use kurbo::Shape as _;

#[test_log::test]
fn test_kurbo_from() {
    log::info!("logging OK");
    let mut path: BezPath = kurbo::Circle{center:kurbo::Point::new(0., 0.), radius:50.0}.to_path(1.0);
    path.extend(kurbo::Rect::new(0., 0., 100., 100.).to_path(1.0));
    /*let mut path2 = BezPath::new();
    path2.move_to((30., 30.));
    path2.curve_to((-35., 80.), (-150., -50.), (-100., -100.));
    path.extend(&path2);*/
    let outline = Outline::<()>::from_kurbo(&path);
    let len1: usize = outline.first().map(|c|c.len()).unwrap_or(0);
    //eprintln!("{:?}", &outline);
    let kpath = outline.clone().into_kurbo().unwrap();
    let outline2 = Outline::<()>::from_kurbo(&kpath);
    let len2: usize = outline2.first().map(|c|c.len()).unwrap_or(0);
    assert_eq!(outline.clone().round_to_int(), outline2.clone().round_to_int());
    let mut glif = Glif::new();
    eprintln!("{:#?}", &path);
    eprintln!("{:#?}", &kpath);
    eprintln!("{:?}", &outline);
    eprintln!("{:?}", &outline2);
    glif.outline = Some(outline.clone());
    println!("{}", glifparser::write(&glif).unwrap());
    glif.outline = Some(outline2.clone());
    println!("{}", glifparser::write(&glif).unwrap());
    assert_eq!(len1, len2);
    assert_eq!(&outline, &outline2);
}
