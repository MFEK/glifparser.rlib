use std::collections::VecDeque;

use super::{GlifOutline, GlifOutlineType, Contour, Outline, OutlineType};
use crate::error::GlifParserError;
use crate::point::{Point, Handle, PointData, PointType, GlifPoint};

use integer_or_float::IntegerOrFloat;
use log::warn;

fn midpoint(x1: IntegerOrFloat, x2: IntegerOrFloat, y1: IntegerOrFloat, y2: IntegerOrFloat) -> (IntegerOrFloat, IntegerOrFloat) {
    ((x1 + x2) / 2., (y1 + y2) / 2.)
}

// UFO uses the same compact format as TTF, so we need to expand it.
pub fn quadratic_outline<PD: PointData>(goutline: &GlifOutline) -> Outline<PD> {
    let mut outline: Outline<PD> = Vec::new();

    let mut temp_outline: VecDeque<VecDeque<GlifPoint>> = VecDeque::new();

    let mut stack: VecDeque<&GlifPoint> = VecDeque::new();

    for gc in goutline.iter() {
        let mut temp_contour = VecDeque::new();

        for gp in gc.iter() {
            match gp.ptype {
                PointType::OffCurve => {
                    stack.push_back(&gp);
                }
                _ => {}
            }

            if stack.len() == 2 {
                let h1 = stack.pop_front().unwrap();
                let h2 = stack.pop_front().unwrap();
                let mp = midpoint(h1.x, h2.x, h1.y, h2.y);

                temp_contour.push_back(h1.clone());
                temp_contour.push_back(GlifPoint::from_x_y_type((mp.0, mp.1), PointType::QCurve).name(gp.name.clone()));
                stack.push_back(h2);
            } else if gp.ptype != PointType::OffCurve {
                let h1 = stack.pop_front();
                match h1 {
                    Some(h) => temp_contour.push_back(h.clone()),
                    _ => {}
                }
                temp_contour.push_back(gp.clone());
            }
        }
        if let (Some(h1), Some(h2)) = (stack.pop_front(), temp_contour.get(0)) {
            let mp = midpoint(h1.x, h2.x, h1.y, h2.y);
            let (t, tx, ty) = (h2.ptype, h2.x, h2.y);
            let (xy, ptype) = if t == PointType::OffCurve {
                ((mp.0, mp.1), PointType::QCurve)
            } else {
                ((tx, ty), PointType::QClose) // TODO: Change to QCurve & vigorously test quadratic, often ignored
            };
            temp_contour.push_back(h1.clone());
            temp_contour.push_back(GlifPoint::from_x_y_type(xy, ptype));
        }

        temp_outline.push_back(temp_contour);
        assert_eq!(stack.len(), 0);
    }

    for gc in temp_outline.iter() {
        let mut contour: Contour<PD> = Vec::new();

        for gp in gc {
            match gp.ptype {
                PointType::OffCurve => {
                    stack.push_back(&gp);
                }
                _ => {
                    assert!(stack.len() < 2);
                    let h1 = stack.pop_front();

                    if let Some(_) = h1 {
                        contour.last_mut().map(|p| p.a = Handle::from(h1));
                    }

                    let (x, y) = (gp.x.into(), gp.y.into());

                    contour.push(Point {
                        x, y,
                        smooth: gp.smooth,
                        name: gp.name.clone(),
                        ptype: gp.ptype,
                        .. Default::default()
                    });
                }
            }
        }

        if contour.len() > 0 {
            outline.push(contour);
        } else {
            warn!("Dropped empty contour. Lone `move` point in .glif? GlifContour: {:?}", &gc);
        }
    }

    outline
}

// Stack based outline builder. Push all offcurve points onto the stack, pop them when we see an on
// curve point. For each point, we add one handle to the current point, and one to the last. We
// then connect the last point to the first to make the loop, (if path is closed).
pub fn cubic_outline<PD: PointData>(goutline: &GlifOutline) -> Outline<PD> {
    let mut outline: Outline<PD> = Vec::new();

    let mut stack: VecDeque<&GlifPoint> = VecDeque::new();

    for gc in goutline.iter() {
        let mut contour: Contour<PD> = Vec::new();

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
                        x, y,
                        b: Handle::from(h2),
                        smooth: gp.smooth,
                        name: gp.name.clone(),
                        ptype: gp.ptype,
                        .. Default::default()
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
            warn!("Dropped empty contour. Lone `move` point in .glif? GlifContour: {:?}", &gc);
        }
        else if contour.len() > 0 {
            outline.push(contour);
        }
    }

    outline
}

impl<PD: PointData> TryInto<Outline<PD>> for GlifOutline {
    type Error = GlifParserError;
    fn try_into(mut self) -> Result<Outline<PD>, GlifParserError> {
        if self.otype == GlifOutlineType::default() {
            self.figure_type();
        }
        Ok(match self.otype.into() {
            OutlineType::Cubic => cubic_outline(&self),
            OutlineType::Quadratic => quadratic_outline(&self),
            OutlineType::Spiro => Err(GlifParserError::GlifInputError("Spiro as yet unimplemented".to_string()))?,
        })
    }
}
