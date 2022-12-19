use super::{Contour, Outline};

use crate::contour::{PrevNext as ContourPrevNext, State as ContourState};
use crate::point::{GlifPoint, Handle, Point, PointData, PointType};

use std::collections::VecDeque;

use integer_or_float::IntegerOrFloat::*;

/// Only knocks out _exactly equal_ floats. For colocated within see
/// [`MFEKmath::Fixup::assert_colocated_within`](
/// https://docs.rs/MFEKmath/latest/MFEKmath/trait.Fixup.html#tymethod.assert_colocated_within ).
pub trait RefigurePointTypes<PD: PointData> {
    fn refigure_point_types(&mut self);
}

impl<PD: PointData> RefigurePointTypes<PD> for Outline<PD> {
    fn refigure_point_types(&mut self) {
        for contour in self {
            contour.refigure_point_types();
        }
    }
}

impl<PD: PointData> RefigurePointTypes<PD> for Contour<PD> {
    fn refigure_point_types(&mut self) {
        for i in 0..self.len() {
            if let Handle::At(ax, ay) = self[i].a {
                if ax == self[i].x && ay == self[i].y {
                    self[i].a = Handle::Colocated;
                }
            }
            if let Handle::At(bx, by) = self[i].b {
                if bx == self[i].x && by == self[i].y {
                    self[i].b = Handle::Colocated;
                }
            }
            self[i].ptype = self.point_type_for_idx(i);
        }
    }
}

trait PointTypeForIdx: ContourPrevNext + ContourState {
    fn point_type_for_idx(&self, idx: usize) -> PointType;
}

impl<PD: PointData> PointTypeForIdx for Contour<PD> {
    fn point_type_for_idx(&self, idx: usize) -> PointType {
        let open_contour = self.is_open();
        let point = &self[idx];
        let (prev_a, next_b) =
            if let Ok(((prev_a, _prev_b), (_next_a, next_b))) = self.contour_prev_next_handles(idx) {
                (prev_a, next_b)
            } else {
                return PointType::default();
            };
        if !open_contour || idx != 0 {
            match (prev_a, point.b) {
                (Handle::At(..), Handle::Colocated)
                | (Handle::Colocated, Handle::At(..))
                | (Handle::At(..), Handle::At(..)) => PointType::Curve,
                (Handle::Colocated, Handle::Colocated) => PointType::Line,
            }
        } else if !open_contour && idx == 0 {
            match (point.a, next_b) {
                (Handle::At(..), Handle::Colocated)
                | (Handle::Colocated, Handle::At(..))
                | (Handle::At(..), Handle::At(..)) => PointType::Curve,
                (Handle::Colocated, Handle::Colocated) => PointType::Line,
            }
        } else {
            PointType::Move
        }
    }
}

/// This trait is primarily intended for easing `.glif` equality testing internally by our test
/// suite. It therefore doesn't do any of the fancy things it could like change point types and
/// assert handles as colocated. Consider [`MFEKmath::Fixup`](
/// https://docs.rs/MFEKmath/latest/MFEKmath/trait.Fixup.html ), [`RefigurePointTypes`], etc., not
/// this. (…Or perhaps together?)
pub trait RoundToInt {
    fn round_to_int(&mut self);
}

impl<PD: PointData> RoundToInt for Point<PD> {
    fn round_to_int(&mut self) {
        self.x = self.x.round();
        self.y = self.y.round();
    }
}

impl RoundToInt for GlifPoint {
    fn round_to_int(&mut self) {
        self.x = Integer(f32::from(self.x).round() as i32);
        self.y = Integer(f32::from(self.y).round() as i32);
    }
}

macro_rules! impl_rti {
    ($type:ident) => {
        impl<R: RoundToInt> RoundToInt for $type<R> {
            fn round_to_int(&mut self) {
                for p in self.iter_mut() {
                    p.round_to_int();
                }
            }
        }
    };
}

impl_rti!(Vec);
impl_rti!(VecDeque);
