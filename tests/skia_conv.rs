#![cfg(feature = "skia")]

use skia_safe::{Path, Rect, RRect, Point as SkPoint};
use glifparser::outline::skia::FromSkiaPath;
use glifparser::Outline;

#[test]
fn test_from_skp() {
    let mut path = Path::new();
    path.move_to((50., 50.));
    path.conic_to((50., 50.,), (70., 70.), 0.33);
    path.quad_to((60., 60.,), (50., 50.));
    path.close();
    let rect = Rect::new(0., 0., 100., 100.);
    let rrect = RRect::new_rect_radii(rect, &[SkPoint::new(20., 20.); 4]);
    path.add_rrect(rrect, None);
    path.add_circle((100., 100.), 10., None);
    let rect = Rect::new(0., 0., 30., 190.);
    path.add_oval(&rect, None);
    path.move_to((50., 50.));
    path.conic_to((50., 50.,), (150., 150.), 0.33);
    path.move_to((50., 50.));
    path.quad_to((50., 100.), (100., 200.));
    path.move_to((50., 0.));
    path.cubic_to((50., 50.), (50., -50.), (50., 100.));
    path.cubic_to((50., 150.), (50., -150.), (50., 200.));
    let o: Outline<()> = Outline::from_skia_path(&path);
    assert_eq!(o.len(), 7);
    assert_eq!(o[0][0].x, 50.);
}
