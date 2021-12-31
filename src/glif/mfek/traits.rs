use super::MFEKOutline;
use crate::{outline, point};

pub trait DowngradeOutline<PD: point::PointData> {
    fn downgrade(self) -> outline::Outline<PD>;
    fn cleanly_downgradable(&self) -> bool;
}

pub trait UpgradeOutline<PD: point::PointData> {
    fn upgrade(self) -> MFEKOutline<PD>;
}
