pub use kurbo::Point as KurboPoint;

pub trait FromKurboPoint {
    fn from_kurbo(kp: &kurbo::Point) -> Self;
}

pub trait ToKurboPoint {
    fn to_kurbo(&self) -> KurboPoint;
}
