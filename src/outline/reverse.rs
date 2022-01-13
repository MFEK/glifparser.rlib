use super::contour::Reverse;
use super::Outline;
use crate::point::PointData;

impl<PD: PointData> Reverse for Outline<PD> {
    fn to_reversed(&self) -> Outline<PD> {
        let mut ret = Outline::new();

        for c in self.iter() {
            let new_c = c.clone().to_reversed();
            ret.push(new_c);
        }

        ret
    }
}
