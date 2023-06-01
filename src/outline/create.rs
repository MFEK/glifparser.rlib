use std::collections::VecDeque;

use super::{Contour, GlifContour, GlifOutline, Outline};
use crate::point::{GlifPoint, Handle, Point, PointData, PointType};

use integer_or_float::IntegerOrFloat;
use log::warn;

fn midpoint(
    x1: IntegerOrFloat,
    x2: IntegerOrFloat,
    y1: IntegerOrFloat,
    y2: IntegerOrFloat,
) -> (IntegerOrFloat, IntegerOrFloat) {
    ((x1 + x2) / 2., (y1 + y2) / 2.)
}

pub fn quadratic_contour<PD: PointData>(gc: &GlifContour) -> Contour<PD> {
    let mut contour: Contour<PD> = Vec::new();
    let mut stack: VecDeque<&GlifPoint> = VecDeque::new();

    for gp in gc.iter() {
        match gp.ptype {
            PointType::OffCurve => {
                stack.push_back(gp);
            }
            _ => {}
        }

        if stack.len() == 2 {
            let h1 = stack.pop_front().unwrap();
            let h2 = stack.pop_front().unwrap();
            let mp = midpoint(h1.x, h2.x, h1.y, h2.y);

            contour.push(Point {
                x: h1.x.into(),
                y: h1.y.into(),
                ptype: h1.ptype,
                name: h1.name.clone(),
                smooth: h1.smooth,
                ..Default::default()
            });

            contour.push(Point {
                x: mp.0.into(),
                y: mp.1.into(),
                ptype: PointType::QCurve,
                name: gp.name.clone(),
                ..Default::default()
            });

            stack.push_back(h2);
        } else if gp.ptype != PointType::OffCurve {
            let h1 = stack.pop_front();
            match h1 {
                Some(h) => contour.push(Point {
                    x: h.x.into(),
                    y: h.y.into(),
                    ptype: h.ptype,
                    name: h.name.clone(),
                    smooth: h.smooth,
                    ..Default::default()
                }),
                _ => {}
            }

            contour.push(Point {
                x: gp.x.into(),
                y: gp.y.into(),
                ptype: gp.ptype,
                name: gp.name.clone(),
                smooth: gp.smooth,
                ..Default::default()
            });
        }
    }

    if let Some(h1) = stack.pop_front() {
        if let Some(h2) = contour.get(0) {
            let mp = midpoint(h1.x.into(), h2.x.into(), h1.y.into(), h2.y.into());
            let ptype = if h2.ptype == PointType::OffCurve {
                PointType::QCurve
            } else {
                PointType::QClose
            };

            contour.push(Point {
                x: h1.x.into(),
                y: h1.y.into(),
                ptype: h1.ptype,
                name: h1.name.clone(),
                smooth: h1.smooth,
                ..Default::default()
            });

            contour.push(Point {
                x: mp.0.into(),
                y: mp.1.into(),
                ptype,
                ..Default::default()
            });
        }
    }

    contour
}

pub fn cubic_contour<PD: PointData>(gc: &GlifContour) -> Contour<PD> {
    let mut contour: Contour<PD> = Vec::new();
    let mut stack: VecDeque<&GlifPoint> = VecDeque::new();

    for gp in gc.iter() {
        match gp.ptype {
            PointType::OffCurve => {
                stack.push_back(&gp);
            }
            PointType::Line | PointType::Move | PointType::Curve => {
                let h1 = stack.pop_front();
                let h2 = stack.pop_front();

                contour.last_mut().map(|p| p.a = Handle::from(h1));

                let (x, y) = (gp.x.into(), gp.y.into());

                contour.push(Point {
                    x,
                    y,
                    a: Handle::from(h1),
                    b: Handle::from(h2),
                    smooth: gp.smooth,
                    name: gp.name.clone(),
                    ptype: gp.ptype,
                    ..Default::default()
                });
            }
            PointType::QCurve => {
                unreachable!("Quadratic point in cubic glyph! File is corrupt.")
            }
            _ => {}
        }
    }

    let h1 = stack.pop_front();
    let h2 = stack.pop_front();

    contour.last_mut().map(|p| p.a = Handle::from(h1));

    if contour.len() > 0 && contour[0].ptype != PointType::Move {
        contour.first_mut().map(|p| p.b = Handle::from(h2));
    }

    if contour.len() == 1 && contour.first().unwrap().ptype == PointType::Move {
        warn!(
            "Dropped empty contour. Lone `move` point in .glif? GlifContour: {:?}",
            &gc
        );
    } else if contour.len() > 0 {
        return contour;
    }

    // In case of empty or single move point contours, return an empty contour
    Vec::new()
}

impl<PD: PointData> Into<Outline<PD>> for GlifOutline {
    fn into(self) -> Outline<PD> {
        let mut outline: Outline<PD> = Vec::new();

        for gc in self.contours.iter() {
            let contour: Contour<PD> = match gc[0].ptype {
                PointType::Curve | PointType::Line | PointType::Move => cubic_contour(gc),
                PointType::QCurve | PointType::QClose => quadratic_contour(gc),
                _ => Vec::new(), // Handle other cases as necessary
            };

            if !contour.is_empty() {
                outline.push(contour);
            }
        }

        outline
    }
}
