use super::Contour;
use crate::outline::refigure::RefigurePointTypes as _;
use crate::point::{Handle, Point, PointData, PointType};

// TODO: Implement this trait on the `data` members of ContourOperations.
/// Reverse the logical (visual) order of Bézier splines in a contour, flipping handles as
/// necessary, and taking into account open/closed state.
pub trait Reverse: Sized + Clone {
    fn to_reversed(&self) -> Self;
    /// Following semantics of std::slice::reverse
    /// Performance note: internally makes a copy. Beware overlong contours.
    fn reverse(&mut self) {
        let reversed = self.clone().to_reversed();
        *self = reversed;
    }
}

impl<PD: PointData> Reverse for Contour<PD> {
    fn to_reversed(&self) -> Contour<PD> {
        let mut new_c = Contour::with_capacity(self.len());
        let open_contour = self.first().unwrap().ptype == PointType::Move && self.len() > 1;
        // This is necessary because although Rev and Chain both implement Iterator, they're
        // incompatible types. So, we need to make a mutable reference to a trait object.
        let (mut iter_t1, mut iter_t2);
        let iter: &mut dyn Iterator<Item = &Point<PD>> = if !open_contour {
            iter_t1 = self[0..1].iter().chain(self[1..].iter().rev());
            &mut iter_t1
        } else {
            iter_t2 = self.iter().rev();
            &mut iter_t2
        };

        // Considering the contour in reversed order, reverse a/b and point order.
        for p in iter {
            let mut new_p = p.clone();
            // a is next, b is prev
            let a = p.a;
            let b = p.b;
            new_p.a = b;
            new_p.b = a;
            new_c.push(new_p);
        }

        let c_len = new_c.len();
        // Considering only open contours post-reversal, reverse which point is the Move.
        if open_contour {
            new_c[0].ptype = PointType::Move;
            let ptype = if c_len >= 3 {
                match (new_c[c_len - 2].a, new_c[c_len - 1].b) {
                    (Handle::At(..), _) | (_, Handle::At(..)) => PointType::Curve,
                    (Handle::Colocated, Handle::Colocated) => PointType::Line,
                }
            } else if c_len == 2 {
                match (new_c[0].a, new_c[1].b) {
                    (Handle::At(..), _) | (_, Handle::At(..)) => PointType::Curve,
                    (Handle::Colocated, Handle::Colocated) => PointType::Line,
                }
            } else {
                log::error!("You probably should not be trying to reverse single-point contours. 気をつけてくださいね。");
                return new_c
            };
            new_c[c_len - 1].ptype = ptype;
            debug_assert!(new_c[0].b == Handle::Colocated);
            debug_assert!(new_c[c_len - 1].a == Handle::Colocated);
        }

        new_c
    }
}
