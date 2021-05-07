mod to;
pub use to::*;

mod from;
pub use from::*;

use skia_safe as skia;
use skia::{Point as SkPoint};

// This trait isn't public because it's only for Skia points. An exported version which works on
// glifparser points may be needed some day, PR's welcome.
trait QuadsToCubics {
    fn quads_to_cubics(self) -> [SkPoint; 4];
}

// This method of Quad->Cubic conversion is used all over the place in FontForge.
impl QuadsToCubics for [SkPoint; 3] {
    fn quads_to_cubics(self) -> [SkPoint; 4] {
        #[allow(unused_assignments)]
        let [mut p0, mut p1, mut p2, mut p3] = [SkPoint::default(); 4];
        p0 = self[0];
        p1.x = self[0].x + (2./3.) * (self[1].x-self[0].x);
        p1.y = self[0].y + (2./3.) * (self[1].y-self[0].y);
        p2.x = self[2].x + (2./3.) * (self[1].x-self[2].x);
        p2.y = self[2].y + (2./3.) * (self[1].y-self[2].y);
        p3 = self[2];
        [p0, p1, p2, p3]
    }
}
