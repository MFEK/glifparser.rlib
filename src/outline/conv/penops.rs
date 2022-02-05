use crate::error::GlifParserError;
use crate::point::IsValid;
use crate::point::{GlifPoint, PointData, PointType, WhichHandle};
use PointType::*;
use crate::outline::Contour;
use crate::outline::contour::{PrevNext as _, State as _};
use crate::outline::refigure::RefigurePointTypes as _;

#[derive(Debug, Clone, PartialEq)]
pub enum PenOperations {
    MoveTo(GlifPoint),
    LineTo(GlifPoint),
    QuadTo(GlifPoint, GlifPoint),
    CurveTo(GlifPoint, GlifPoint, GlifPoint),
    Close,
}

use PenOperations::*;

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
            QuadTo(gpb, gp) => gp.ptype != PointType::OffCurve && gp.is_valid() && gpb.ptype == PointType::OffCurve && gpb.is_valid(),
            CurveTo(gpb, gp2a, gp) => gp.ptype != PointType::OffCurve && gp.is_valid() && gpb.ptype == PointType::OffCurve && gpb.is_valid() && gp2a.ptype == PointType::OffCurve && gp2a.is_valid(),
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
                },
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
                    *pen_vec.last_mut().unwrap() = PenOperations::CurveTo(
                        p1, p2, self.first().unwrap().into()
                    );
                }
            }
            pen_vec.push(PenOperations::Close);
        }

        Ok(pen_vec)
    }
}

/*
impl PenOperations {
    pub fn split(self) -> Vec<Self> {
    }
}*/
