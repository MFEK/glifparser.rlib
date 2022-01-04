use super::Contour;

use crate::contour::{PrevNext as ContourPrevNext, State as ContourState};
use crate::point::{Handle, PointData, PointType};

pub trait RefigurePointTypes<PD: PointData>: ContourPrevNext + ContourState {
    fn refigure_point_types(&mut self);
    fn point_type_for_idx(&self, idx: usize) -> PointType;
}

impl<PD: PointData> RefigurePointTypes<PD> for Contour<PD> {
    fn refigure_point_types(&mut self) {
        for i in 0..self.len() {
            // Only knocks out exactly equal floats. For colocated within see MFEKmath
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
    fn point_type_for_idx(&self, idx: usize) -> PointType {
        let open_contour = self.is_open();
        let point = &self[idx];
        let prev_a =
            if let Ok(((prev_a, _prev_b), (_next_a, _next_b))) = self.contour_prev_next_handles(idx) {
                prev_a
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
        } else {
            PointType::Move
        }
    }
}
