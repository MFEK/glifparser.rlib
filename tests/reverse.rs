use glifparser::outline::{Contour, Outline};
use glifparser::outline::Reverse as _;
use glifparser::outline::FromKurbo as _;
use kurbo::Shape as _;

#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref CIRCLE: Outline::<()> = Outline::<()>::from_kurbo(&kurbo::BezPath::from_vec(kurbo::Circle::new(kurbo::Point::new(0.0, 0.0), 50.0).path_elements(f64::EPSILON).collect()));
}
static ITERATIONS: usize = 5_000;

#[test]
fn test_rev_outline() {
    let gcircle = CIRCLE.clone();
    let mut gcircle2 = CIRCLE.clone();
    for _ in 0 .. ITERATIONS {
        gcircle2.reverse();
    }
    assert_eq!(gcircle, gcircle2);
    let mut gcircle2 = CIRCLE.clone();
    for _ in 0 .. ITERATIONS - 1 {
        gcircle2.reverse();
    }
    assert_ne!(gcircle, gcircle2);
}

#[test]
fn test_rev_contour() {
    let gcircle = &*CIRCLE;
    let gcircle_contour: Contour<()> = gcircle.clone().pop().unwrap();
    let mut gcircle2_contour: Contour<()> = gcircle_contour.clone();
    for _ in 0 .. ITERATIONS {
        gcircle2_contour.reverse();
    }
    assert_eq!(gcircle_contour, gcircle2_contour);
    let mut gcircle2_contour: Contour<()> = gcircle_contour.clone();
    for _ in 0 .. ITERATIONS - 1 {
        gcircle2_contour.reverse();
    }
    assert_ne!(gcircle_contour, gcircle2_contour);
}
