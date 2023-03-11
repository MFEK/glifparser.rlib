use crate::PointData;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MFEKGuidelineInfo {
    pub fixed: bool,
    pub format: bool,
    pub right: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MFEKPointData {
    Guideline(MFEKGuidelineInfo),
}

impl Default for MFEKPointData {
    fn default() -> Self {
        MFEKPointData::Guideline(MFEKGuidelineInfo::default())
    }
}

impl MFEKPointData {
    pub fn new_guideline_data(fixed: bool, format: bool, right: bool) -> Self {
        Self::Guideline(MFEKGuidelineInfo {
            fixed,
            format,
            right,
        })
    }
    pub fn as_guideline(&self) -> MFEKGuidelineInfo {
        #[allow(irrefutable_let_patterns)]
        if let MFEKPointData::Guideline(guide) = self {
            *guide
        } else {
            panic!("Tried to unwrap non-guideline as guideline")
        }
    }
}

impl PointData for MFEKPointData {}