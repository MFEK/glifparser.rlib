use glifparser::{Contour, Handle, Point, PointType, contour::CheckSmooth};

#[test]
fn smooth() {
    let mut contour: Contour<()> = vec![Point::from_x_y_type((0., 0.), PointType::Move), Point::from_x_y_a_b_type((500., 500.), (Handle::At(250., 250.), Handle::At(750., 750.)), PointType::Curve), Point::from_x_y_type((1000., 1000.), PointType::Curve)];
    let smooth = contour.is_point_smooth(1usize);
    contour.check_smooth(1usize).unwrap();
    assert!(smooth.unwrap() && contour[1].smooth);
    let mut contour: Contour<()> = vec![Point::from_x_y_type((0., 0.), PointType::Move), Point::from_x_y_a_b_type((500., 500.), (Handle::At(250., 550.), Handle::At(750., 250.)), PointType::Curve), Point::from_x_y_type((1000., 1000.), PointType::Curve)];
    let smooth = contour.is_point_smooth(1usize);
    assert!(!smooth.unwrap());
    contour[1].smooth = true;
    contour.check_smooth(1usize).unwrap();
    assert!(!contour[1].smooth);
}
