use super::{*, contour_operations::ContourOperations};
use crate::{outline, point};

use std::collections::VecDeque;

pub type MFEKOutlineContourOperations<PD> = VecDeque<Option<ContourOperations<PD>>>;
/// Vec (layers) → Vec (contours) → which may or may not have any operations
/// ```
/// # use glifparser::glif::mfek::ContourOperations;
/// # fn doctest<PD: glifparser::PointData>() ->
/// Vec<Vec<Option<ContourOperations<PD>>>>
/// # { vec![] }
/// ```
pub type MFEKGlifContourOperations<PD> = VecDeque<MFEKOutlineContourOperations<PD>>;
impl<PD: PointData> PointData for MFEKOutlineContourOperations<PD> {}
impl<PD: PointData> PointData for MFEKGlifContourOperations<PD> {}

pub trait DowngradeOutline<PD: point::PointData> {
    fn downgrade(self) -> outline::Outline<PD>;
    fn cleanly_downgradable(&self) -> bool;
}

pub trait UpgradeOutline<PD: point::PointData> {
    fn upgrade(self) -> MFEKOutline<PD>;
}

pub trait ManageContourOperations {
    type Output: PointData;
    /// Meant for cleanly re-adding previously culled ops
    fn upgrade_contour_ops(&mut self, ops: Self::Output) -> Result<(), UpgradeContourOpsError>;
    /// Returns data receivable by ``upgrade_contour_ops``
    fn downgrade_contour_ops(&mut self) -> Self::Output;
}

impl<PD: PointData> DowngradeOutline<PD> for MFEKOutline<PD> {
    fn cleanly_downgradable(&self) -> bool {
        for contour in self {
            match contour.inner {
                MFEKContourInner::Cubic(_) => {},
                _ => return false
            }
        }
        
        self.iter().all(|c| c.operation.is_none())
    }

    fn downgrade(self) -> Outline<PD> {
        let mut ret = Outline::new();
        for contour in self {
            match contour.inner {
                MFEKContourInner::Cubic(cubic_contour) => {
                    ret.push(cubic_contour);
                }
                _ => panic!()
            }
        }
        ret
    }
}

impl<PD: PointData> UpgradeOutline<PD> for Outline<PD> {
    fn upgrade(self) -> MFEKOutline<PD> {
        let mut ret = MFEKOutline::new();
        for contour in &self {
            ret.push(contour.into());
        }
        ret
    }
}

impl<PD: PointData> ManageContourOperations for MFEKOutline<PD> {
    type Output = MFEKOutlineContourOperations<PD>;
    fn upgrade_contour_ops(&mut self, mut ops: Self::Output) -> Result<(), UpgradeContourOpsError> {
        let mut iter = self.iter_mut();
        while let Some(mut contour) = iter.next() {
            contour.operation = ops
                .pop_front()
                .ok_or(UpgradeContourOpsError::MoreContoursThanOps)?;
        }
        if !ops.is_empty() {
            ops.clear();
            Err(UpgradeContourOpsError::MoreOpsThanContours)
        } else {
            Ok(())
        }
    }
    fn downgrade_contour_ops(&mut self) -> Self::Output {
        let mut o_vec = VecDeque::with_capacity(self.len());
        for contour in self.iter_mut() {
            o_vec.push_back(contour.operation.take());
        }
        o_vec
    }
}

impl<PD: PointData> ManageContourOperations for MFEKGlif<PD> {
    type Output = MFEKGlifContourOperations<PD>;
    fn upgrade_contour_ops(&mut self, mut ops: Self::Output) -> Result<(), UpgradeContourOpsError> {
        let mut iter = self.layers.iter_mut();
        while let Some(ref mut outline) = iter.next().map(|layer| &mut layer.outline) {
            outline.upgrade_contour_ops(
                ops.pop_front()
                    .ok_or(UpgradeContourOpsError::MoreLayersThanVecOps)?,
            )?;
        }
        if !ops.is_empty() {
            ops.clear();
            Err(UpgradeContourOpsError::MoreVecOpsThanLayers)
        } else {
            Ok(())
        }
    }

    fn downgrade_contour_ops(&mut self) -> Self::Output {
        let mut layers: Vec<_> = self.layers.drain(..).collect();
        let mut l_vec = VecDeque::with_capacity(layers.len());
        for layer in layers.iter_mut() {
            l_vec.push_back(layer.outline.downgrade_contour_ops());
        }
        self.layers = layers;
        l_vec
    }
}
