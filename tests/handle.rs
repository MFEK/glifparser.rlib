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

#[test]
fn test_which_handle() {
    let wh1: WhichHandle = 'A'.into();
    assert_eq!(wh1, WhichHandle::A);
    let wh1: WhichHandle = 'Ａ'.into();
    assert_eq!(wh1, WhichHandle::A);
    let wh1: WhichHandle = "Ａ".into();
    assert_eq!(wh1, WhichHandle::A);
    let wh1: WhichHandle = "b".into();
    assert_eq!(wh1, WhichHandle::B);
    let wh1: WhichHandle = '\u{0}'.into();
    assert_eq!(wh1, WhichHandle::Neither);
}

#[cfg(not(debug_assertions))]
#[test]
fn test_which_handle_nonstrict() {
    use std::str::FromStr as _;

    let wh1: WhichHandle = " b ".into();
    assert_eq!(wh1, WhichHandle::B);
    let wh1: WhichHandle = 'C'.into();
    assert_eq!(wh1, WhichHandle::Neither);
    let wh1 = WhichHandle::from_str("");
    assert!(wh1.is_err());
    let wh1 = WhichHandle::from_str("   BBBB  ");
    assert_eq!(wh1, Ok(WhichHandle::B));
}
