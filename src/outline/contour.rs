mod xml;

use kurbo;
use kurbo::ParamCurveNearest as _;

use crate::error::GlifParserError;
use crate::point::{Handle, PointData, PointType};

use super::{Contour, GlifContour};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum End {
    Head,
    Tail,
}

pub trait GenericPrevNext {
    type Error;
    /// Return the previous and next index, given an index.
    fn prev_next(&self, idx: usize) -> Result<(usize, usize), Self::Error>;
    /// Returns which end we have, given an index.
    fn idx_which_end(&self, idx: usize) -> Result<Option<End>, Self::Error>;
    /// Returns whether the index is sane given the vec it's called on, intended to be used with
    /// `?` shortcut.
    fn idx_sane(&self, idx: usize) -> Result<(), Self::Error>;

    fn prev(&self, idx: usize) -> Result<usize, Self::Error> {
        let (prev, _next) = self.prev_next(idx)?;
        Ok(prev)
    }
    fn next(&self, idx: usize) -> Result<usize, Self::Error> {
        let (_prev, next) = self.prev_next(idx)?;
        Ok(next)
    }

    fn idx_at_start_or_end(&self, idx: usize) -> Result<bool, Self::Error> {
        let end = self.idx_which_end(idx)?;
        Ok(end.is_some())
    }
    fn idx_is_sane(&self, idx: usize) -> bool {
        match self.idx_sane(idx) {
            Ok(()) => true,
            Err(_e) => false,
        }
    }
    fn idx_is_insane(&self, idx: usize) -> bool {
        !self.idx_is_sane(idx)
    }
}

pub trait PrevNext {
    type Error;
    /// Return the previous and next index, given an index. As opposed to ``GenericPrevNext``, this always
    /// considers a contour's open/closed state (`assert!(self[0].ptype == PointType::Move)`).
    fn contour_prev_next(&self, idx: usize) -> Result<(Option<usize>, Option<usize>), GlifParserError>;
    fn contour_prev_next_handles(&self, idx: usize) -> Result<((Handle, Handle), (Handle, Handle)), GlifParserError>;
}

impl<T> GenericPrevNext for Vec<T> {
    type Error = GlifParserError;

    fn idx_sane(&self, idx: usize) -> Result<(), GlifParserError> {
        if idx >= self.len() {
            return Err(GlifParserError::PointIdxOutOfBounds { idx, len: self.len() })
        } else if self.len() == 1 {
            return Err(GlifParserError::ContourLenOneUnexpected)
        } else if self.len() == 0 {
            return Err(GlifParserError::ContourLenZeroUnexpected)
        }
        Ok(())
    }

    fn prev_next(&self, idx: usize) -> Result<(usize, usize), GlifParserError> {
        self.idx_sane(idx)?;

        let prev = if idx == 0 {
            self.len() - 1
        } else {
            idx - 1
        };

        let next = if idx == self.len() - 1 {
            0
        } else {
            idx + 1
        };

        Ok((prev, next))
    }

    fn idx_which_end(&self, idx: usize) -> Result<Option<End>, GlifParserError> {
        self.idx_sane(idx)?;
        Ok(if idx == 0 {
            Some(End::Head)
        } else if idx == self.len() - 1 {
            Some(End::Tail)
        } else {
            None
        })
    }
}

pub trait State {
    fn is_open(&self) -> bool;
    fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

impl<PD: PointData> State for Contour<PD> {
    fn is_open(&self) -> bool {
        match self.len() {
            0 | 1 => true,
            _ => self[0].ptype == PointType::Move
        }
    }
}

impl<PD: PointData> PrevNext for Contour<PD> {
    type Error = GlifParserError;
    /// Error will always be GlifParserError::PointIdxOutOfBounds
    fn contour_prev_next(&self, idx: usize) -> Result<(Option<usize>, Option<usize>), GlifParserError> {
        let (prev, next) = self.prev_next(idx)?;
        if self.is_open() && self.idx_at_start_or_end(idx)? {
            let end = self.idx_which_end(idx)?.expect("self.idx_at_start_or_end true but self.idx_which_end returned None???");
            match end {
                End::Head => Ok((None, Some(next))),
                End::Tail => Ok((Some(prev), None)),
            }
        } else {
            Ok((Some(self.prev(idx)?), Some(self.next(idx)?)))
        }
    }
    fn contour_prev_next_handles(&self, idx: usize) -> Result<((Handle, Handle), (Handle, Handle)), GlifParserError> {
        let (prev, next) = self.contour_prev_next(idx)?;
        let prev = prev.map(|idx| (self[idx].a, self[idx].b)).unwrap_or((Handle::Colocated, Handle::Colocated));
        let next = next.map(|idx| (self[idx].a, self[idx].b)).unwrap_or((Handle::Colocated, Handle::Colocated));
        Ok((prev, next))
    }
}

pub trait CheckSmooth {
    fn is_point_smooth_within(&self, idx: usize, within: f32) -> Result<bool, GlifParserError>;
    fn is_point_smooth(&self, idx: usize) -> Result<bool, GlifParserError> {
        self.is_point_smooth_within(idx, 0.01)
    }
    fn check_smooth(&mut self, idx: usize) -> Result<bool, GlifParserError>;
}

impl<PD: PointData> CheckSmooth for Contour<PD> {
    fn is_point_smooth_within(&self, idx: usize, within: f32) -> Result<bool, GlifParserError> {
        let end = self.idx_which_end(idx)?;
        let (prev, next) = self.contour_prev_next(idx)?;
        let p = &self[idx];
        // a is next, b is prev
        let (a, b) = match end {
            None => (p.a, p.b),
            Some(End::Head) => {
                if let Some(prev) = prev {
                    (p.a, self[prev].b)
                } else {
                    return Ok(false)
                }
            }
            Some(End::Tail) => {
                if let Some(next) = next {
                    (self[next].a, p.b)
                } else {
                    return Ok(false)
                }
            }
        };
        let kp0 = kurbo::Point::new(p.x as f64, p.y as f64);
        let (kp1, kp2) = match (a, b) {
            (Handle::At(x1, y1), Handle::At(x2, y2)) => (kurbo::Point::new(x1 as f64, y1 as f64), kurbo::Point::new(x2 as f64, y2 as f64)),
            _ => return Ok(false)
        };
        let line = kurbo::Line::new(kp1, kp2);
        let nearest = line.nearest(kp0, 0.000001);
        if nearest.distance_sq.sqrt() <= within as f64 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn check_smooth(&mut self, idx: usize) -> Result<bool, GlifParserError> {
        let smooth = self.is_point_smooth(idx)?;
        self[idx].smooth = smooth;
        Ok(smooth)
    }
}
