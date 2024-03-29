use super::ToOutline;
use crate::error::GlifParserError;
use crate::outline::contour::{PrevNext as _, State as _};
use crate::outline::refigure::RefigurePointTypes as _;
use crate::outline::{Contour, Outline};
use crate::point::IsValid;
use crate::point::{GlifPoint, Handle, Point, PointData, PointType, WhichHandle};
use PointType::*;

/// Representation of glyph data as pen operations, rather than as [`Point`]'s.
#[derive(Debug, Clone, PartialEq, IsVariant, Unwrap)]
pub enum PenOperations {
    /// Should always be used even if first point at `(0. , 0.)`.
    MoveTo(GlifPoint),
    LineTo(GlifPoint),
    QuadTo(GlifPoint, GlifPoint),
    CurveTo(GlifPoint, GlifPoint, GlifPoint),
    Close,
}

use PenOperations::*;

impl PenOperations {
    pub fn len(&self) -> usize {
        match self {
            MoveTo(..) => 1,
            LineTo(..) => 1,
            QuadTo(..) => 2,
            CurveTo(..) => 3,
            Close => 0,
        }
    }

    pub fn simplify(&mut self) {
        if let CurveTo(pa, pb, p) = self {
            if p.x == pa.x && p.y == pa.y && p.x == pb.x && p.y == pb.y {
                *self = Self::LineTo(p.clone());
            }
        }
    }
}

impl Default for PenOperations {
    /// By default, create a move_to to origin.
    fn default() -> Self {
        PenOperations::MoveTo(GlifPoint::from_x_y_type((0., 0.), PointType::Move))
    }
}

impl IsValid for PenOperations {
    fn is_valid(&self) -> bool {
        match self {
            MoveTo(gp) => gp.ptype == PointType::Move && gp.is_valid(),
            LineTo(gp) => gp.ptype == PointType::Line && gp.is_valid(),
            QuadTo(gpb, gp) => {
                gp.ptype != PointType::OffCurve
                    && gp.is_valid()
                    && gpb.ptype == PointType::OffCurve
                    && gpb.is_valid()
            }
            CurveTo(gpb, gp2a, gp) => {
                gp.ptype != PointType::OffCurve
                    && gp.is_valid()
                    && gpb.ptype == PointType::OffCurve
                    && gpb.is_valid()
                    && gp2a.ptype == PointType::OffCurve
                    && gp2a.is_valid()
            }
            Close => true,
        }
    }
}

impl Into<Vec<GlifPoint>> for PenOperations {
    fn into(self) -> Vec<GlifPoint> {
        match self {
            MoveTo(gp) => vec![gp],
            LineTo(gp) => vec![gp],
            QuadTo(gpb, gp) => vec![gpb, gp],
            CurveTo(gpb, gp2a, gp) => vec![gpb, gp2a, gp],
            Close => vec![],
        }
    }
}

pub trait IntoPenOperations {
    fn into_pen_operations(&mut self) -> Result<Vec<PenOperations>, GlifParserError>;
}

impl<PD: PointData> IntoPenOperations for Contour<PD> {
    // mutable as it triggers a refigure!
    fn into_pen_operations(&mut self) -> Result<Vec<PenOperations>, GlifParserError> {
        let is_closed = self.is_closed();
        self.refigure_point_types();
        let mut pen_vec = vec![];

        if is_closed {
            pen_vec.push(PenOperations::MoveTo(self.first().unwrap().into()));
        }

        for (pi, point) in self.iter().enumerate() {
            let p = match point.ptype {
                PointType::Move => PenOperations::MoveTo(point.into()),
                PointType::Line => {
                    if pi == 1 {
                        pen_vec.pop().unwrap();
                        PenOperations::LineTo(point.into())
                    } else {
                        PenOperations::LineTo(point.into())
                    }
                }
                PointType::QCurve => {
                    PenOperations::QuadTo(GlifPoint::from_handle(&point, WhichHandle::A), point.into())
                }
                PointType::Curve => match self.contour_prev_next(pi).unwrap() {
                    (_, Some(next)) => PenOperations::CurveTo(
                        GlifPoint::from_handle(&point, WhichHandle::A),
                        GlifPoint::from_handle(&self[next], WhichHandle::B),
                        (&self[next]).into(),
                    ),
                    (Some(prev), None) => PenOperations::CurveTo(
                        GlifPoint::from_handle(&self[prev], WhichHandle::A),
                        GlifPoint::from_handle(&point, WhichHandle::B),
                        point.into(),
                    ),
                    (None, None) => unreachable!(),
                },
                ptype => return Err(GlifParserError::GlifContourHasBadPointType { pi, ptype }),
            };
            pen_vec.push(p);
        }

        if is_closed {
            if self.last().unwrap().ptype == Curve && self.first().unwrap().ptype == Curve {
                let lp = pen_vec.last().unwrap().clone();
                if let PenOperations::CurveTo(p1, p2, _p3) = lp {
                    *pen_vec.last_mut().unwrap() =
                        PenOperations::CurveTo(p1, p2, self.first().unwrap().into());
                }
            }
            pen_vec.push(PenOperations::Close);
        }

        Ok(pen_vec)
    }
}

/// A list of pen operations that should contain one and only one contour.
pub type PenOperationsContour = Vec<PenOperations>;
/// A list of lists of pen operations creating an outline of [`Vec::len()`] contours.
pub type PenOperationsPath = Vec<PenOperationsContour>;

/// Split a long vec of pen operations into constitutent contours.
pub trait SplitPenOperations: Clone {
    fn split_pen_operations(&self) -> PenOperationsPath;
    fn has_n_contours(&self, n: usize) -> bool {
        self.clone().split_pen_operations().len() == n
    }
}

impl IsValid for PenOperationsContour {
    fn is_valid(&self) -> bool {
        self.has_n_contours(1)
    }
}

impl SplitPenOperations for PenOperationsContour {
    fn split_pen_operations(&self) -> PenOperationsPath {
        let mut koutline = vec![];
        let mut kcontour = vec![];
        
        // if the contour doesn't have more than one point return an empty outline
        if self.len() <= 1 {
            koutline.push(kcontour);
            return koutline
        }
        
        for p in self.iter() {
            let kpv = &p;
            if p.is_move_to() {
                if kcontour.len() > 0 {
                    koutline.push(kcontour);
                }
                kcontour = vec![p.clone()];
            } else if kpv.len() > 0 {
                kcontour.push(p.clone());
            // NB: This can create in ToOutline two points on top of one another, one with a one with b set…
            } else if kpv.len() == 0 {
                let removed = kcontour.remove(0);
                let rm = removed.unwrap_move_to();
                kcontour.insert(0, PenOperations::CurveTo(rm.clone(), rm.clone(), rm.clone()));
            }
        }

        if kcontour.len() > 0 {
            koutline.push(kcontour);
        }

        koutline
    }
}

impl<PD: PointData> ToOutline<PD> for PenOperationsPath {
    fn to_outline(&self) -> Outline<PD> {
        let mut ret: Outline<PD> = Outline::new();

        for skc in self.iter() {
            let skc_len = skc.len();
            let mut contour: Contour<PD> = Contour::new();
            let mut next_points: Vec<GlifPoint>;
            for (i, el) in skc.iter().enumerate() {
                let points: Vec<GlifPoint> = el.clone().into();
                if points.len() == 0 {
                    continue;
                }
                if i != skc_len - 1 {
                    next_points = skc[i + 1].clone().into();
                } else {
                    next_points = skc[0].clone().into();
                }

                let mut point = Point::<PD> {
                    name: None,
                    data: None,
                    x: points[0].x.into(),
                    y: points[0].y.into(),
                    smooth: false,
                    // These will be fixed below, if needed
                    a: Handle::Colocated,
                    b: Handle::Colocated,
                    ptype: (el).into(),
                };

                match point.ptype {
                    PointType::Move => {
                    }
                    PointType::Curve => {
                        if next_points.len() == 3 {
                            point.a = Handle::At(next_points[2].x.into(), next_points[2].y.into());
                        }
                        if let Some(p) = points.get(1) {
                            point.b = Handle::At(p.x.into(), p.y.into());
                        } else {
                            log::warn!("Expected a next handle that does not exist")
                        }
                    }
                    PointType::Line => {
                        if next_points.len() == 3 {
                            // Lines aren't allowed to have off-curve points in glif format
                            point.ptype = PointType::Curve;
                            point.a = Handle::At(next_points[2].x.into(), next_points[2].y.into());
                        }
                    }
                    _ => unreachable!("Got an illegal point type {:?}", point.ptype),
                }
                contour.push(point);
            }

            if contour.len() > 1 {
                let first = contour.first().unwrap();
                let last = contour.last().unwrap();
                let (x, y, _a, b) = (last.x, last.y, last.a, last.b);
                if x == first.x && y == first.y {
                    contour.pop().unwrap();
                    contour.first_mut().unwrap().b = b;
                }
            }
            ret.push(contour);
        }

        ret
    }
}
