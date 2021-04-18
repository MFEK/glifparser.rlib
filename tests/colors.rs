use integer_or_float::IntegerOrFloat::*;

use glifparser;

#[test]
fn test_color() {
    let color: glifparser::Color = "0.5,0.5,1,1".parse().unwrap();
    assert_eq!(color, glifparser::Color { r: Float(0.5), g: Float(0.5), b: Integer(1), a: Integer(1) });
}
