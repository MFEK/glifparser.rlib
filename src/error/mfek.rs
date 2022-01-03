use std::error::Error;
use std::fmt;

#[cfg(feature = "glifserde")]
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UpgradeContourOpsError {
    MoreContoursThanOps,
    MoreOpsThanContours,
    MoreLayersThanVecOps,
    MoreVecOpsThanLayers,
}

impl fmt::Display for UpgradeContourOpsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let (over, under) = match self {
            Self::MoreContoursThanOps => ("contours in outline", "contour operations in list"),
            Self::MoreOpsThanContours => ("contour operations in list", "contours in outline"),
            Self::MoreLayersThanVecOps => ("layers in glyph", "lists of contour operations in list"),
            Self::MoreVecOpsThanLayers => ("lists of contour operations in list", "layers in glyph"),
        };
        write!(
            f,
            "Mismatch while upgrading contour ops: more {} than {}!",
            over, under
        )
    }
}

impl Error for UpgradeContourOpsError {}
