use super::Contour;
use crate::point::{Handle, Point, PointData, PointType};

// TODO: Implement this trait on the `data` members of ContourOperations.
/// Reverse the logical (visual) order of Bézier splines in a contour, flipping handles as
/// necessary, and taking into account open/closed state.
pub trait Reverse {
    fn reverse(self) -> Self;
}

impl<PD: PointData> Reverse for Contour<PD> {
    fn reverse(self) -> Contour<PD> {
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
            new_p.ptype = if a != Handle::Colocated {
                PointType::Curve
            } else {
                PointType::Line
            };
            new_c.push(new_p);
        }

        // Considering only open contours post-reversal, reverse which point is the Move.
        if open_contour {
            new_c[0].ptype = PointType::Move;
            let c_len = new_c.len();
            let ptype = match new_c[c_len - 2].a {
                Handle::At(_, _) => PointType::Curve,
                Handle::Colocated => PointType::Line,
            };
            new_c[c_len - 1].ptype = ptype;
        }

        new_c
    }
}
