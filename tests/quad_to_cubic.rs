use glifparser::glif::IntoXML as _;
use glifparser::outline::{GlifOutline, QuadToCubic as _};
use glifparser::point::{GlifPoint, PointType};
use glifparser::write;
use glifparser::{Glif, Handle, Outline, Point};

static PARABOLA: [[Point<()>; 2]; 1] = {
    use Handle::*;
    use PointType::*;
    [[
        Point {
            b: Colocated,
            x: 0.0,
            y: 0.0,
            a: At(66.66667, 333.33334),
            name: None,
            ptype: Move,
            smooth: false,
            data: None,
        },
        Point {
            b: At(133.33333, 333.33334),
            x: 200.0,
            y: 0.0,
            a: Colocated,
            name: None,
            ptype: Curve,
            smooth: false,
            data: None,
        },
    ]]
};

#[test]
fn parabola() {
    let gp_s = GlifPoint::from_x_y_type((0., 0.), PointType::Move);
    let gp_o = GlifPoint::from_x_y_type((100., 500.), PointType::OffCurve);
    let gp_e = GlifPoint::from_x_y_type((200., 0.), PointType::Curve);
    let result: GlifOutline =
        vec![[gp_s, gp_o, gp_e.clone()].quad_to_cubic().into_iter().collect()].into();
    let result: Outline<()> = result.try_into().unwrap();
    let mut glif = Glif::new();
    assert_eq!(&result, &PARABOLA);
    glif.outline = Some(result);
    //eprintln!("{}", write(&glif).unwrap());
    write(&glif).unwrap();
}
