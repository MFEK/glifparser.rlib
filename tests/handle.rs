use glifparser::{Handle, Point, PointType, WhichHandle};

#[test]
fn test_handles() {
    let point: Point<()> = Point::from_x_y_a_b_type((0.0, 0.0), (Handle::At(1.0, 0.0), Handle::At(0.0, 1.0)), PointType::Move);
    assert_eq!(point.a, Handle::At(1.0, 0.0));
    assert_eq!(point.handle(WhichHandle::A), Handle::At(1.0, 0.0));
    assert_eq!(point.handle(WhichHandle::Neither), Handle::Colocated);

    let mut point = point;
    point.set_handle(WhichHandle::A, Handle::Colocated);
    assert_eq!(point.handle(WhichHandle::A), Handle::Colocated);

    let wh = WhichHandle::A;
    assert_eq!(wh.opposite(), WhichHandle::B);
    assert_eq!(wh.opposite().opposite(), WhichHandle::A);
}
