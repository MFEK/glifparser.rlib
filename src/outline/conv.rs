use super::{Contour, GlifContour};

use crate::point::{GlifPoint, PointData, PointType, WhichHandle};

use std::collections::{HashSet, VecDeque};

pub trait IntoGlifPoints {
    type Output;
    fn into_glifpoints(self) -> Self::Output;
}

// Handle implementations for Outline/GlifOutline/MFEKOutline (in future)
macro_rules! impl_igp {
    ($typ:ident) => {
        impl<IGP: IntoGlifPoints> IntoGlifPoints for $typ<IGP> {
            type Output = $typ<<IGP as IntoGlifPoints>::Output>;
            fn into_glifpoints(self) -> Self::Output {
                self.into_iter().map(|it|it.into_glifpoints()).collect()
            }
        }
    }
}
impl_igp!(Vec);
impl_igp!(VecDeque);

pub(crate) fn cleanup_offcurves(contour: &mut Vec<GlifPoint>) {
    let types: Vec<_> = contour.iter().enumerate().filter_map(|(i, gp)|(gp.ptype != PointType::OffCurve).then(||(i, gp.ptype))).collect();
    debug_assert!(types.iter().all(|(_, t)|t.is_valid_oncurve()));

    let keep_indices: HashSet<usize> = types.into_iter().map(|(i, pt)| {
        let i = i as isize;
        match pt {
            PointType::Move => vec![i],
            PointType::Line => vec![i],
            PointType::QCurve => vec![i-1, i],
            PointType::Curve => vec![i-2, i-1, i],
            pt => panic!("PointType {:?} should be impossible when trying to convert back to an XML <outline>", pt),
        }
    }).flatten().filter_map(|i| (i >= 0).then(||i.try_into().unwrap())).collect();
    let keep: Vec<bool> = (0..contour.len()).map(|idx| keep_indices.contains(&idx)).collect();
    debug_assert_eq!(contour.len(), keep.len());
    let mut iter = keep.iter();
    contour.retain(|_|*iter.next().unwrap());
    debug_assert_eq!(contour.len(), keep_indices.len());
}

impl<PD: PointData> IntoGlifPoints for Contour<PD> {
    type Output = GlifContour;
    fn into_glifpoints(self) -> Self::Output {
        let contour_len = self.len();
        let mut on_points: VecDeque<GlifPoint> = self.iter().map(|p|GlifPoint::from(p)).collect();
        let mut next_handles: VecDeque<GlifPoint> = self.iter().map(|p|GlifPoint::from_handle(p, WhichHandle::A)).collect();
        let mut prev_handles: VecDeque<GlifPoint> = self.iter().map(|p|GlifPoint::from_handle(p, WhichHandle::B)).collect();
        next_handles.rotate_right(1);
        debug_assert!(on_points.len() == next_handles.len() && next_handles.len() == prev_handles.len());
        let mut drains = [next_handles.drain(..), prev_handles.drain(..), on_points.drain(..)];
        let mut glifpoints: Vec<GlifPoint> = std::iter::repeat(()).take(contour_len).fold(Vec::with_capacity(contour_len), |mut acc, _|{acc.extend(drains.iter_mut().map(|d|d.next().unwrap())); acc});
        debug_assert!(drains.into_iter().all(|mut d|d.next().is_none()));
        cleanup_offcurves(&mut glifpoints);
        while let PointType::OffCurve = glifpoints[0].ptype {
            glifpoints.rotate_right(1);
        }
        glifpoints
    }
}
