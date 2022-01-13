use super::contour::Reverse;
use super::Outline;

impl<PD: PointData> Reverse for Outline<PD> {
    fn reverse(self) -> Outline<PD> {
        let mut ret = Outline::new();

        for c in self.iter() {
            let new_c = c.clone().reverse();
            ret.push(new_c);
        }

        ret
    }
}
