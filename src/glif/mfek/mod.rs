use core::panic;
use std::collections::HashSet;
use std::path as stdpath;

#[cfg(feature = "skia")]
use skia_safe::{self as skia, Path};
use kurbo::Affine;
use serde::{Serialize, Deserialize};

use crate::{PointType, Point};
use crate::anchor::Anchor;
use crate::component::{ComponentRect, GlifComponents};
use crate::glif::Glif;
use crate::guideline::Guideline;
use crate::outline::Outline;
#[cfg(feature = "skia")]
use crate::outline::skia::{SkiaPaths, SkiaPointTransforms, ToSkiaPath, ToSkiaPaths};
use crate::point::PointData;

#[macro_use] pub mod layer;
pub use layer::Layer;
pub(crate) use DEFAULT_LAYER_FORMAT_STR;
pub mod traits;
pub mod contour_operations;
pub mod contour;
pub mod point;
pub mod pointdata;
pub mod inner;
pub use contour::MFEKContour;

/// This is an intermediary form used in MFEKglif and other tools. You can .into() a glif into this
/// make changes to MFEK data and then turn it back into a standard UFO glif before saving.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MFEKGlif<PD: PointData> {
    pub layers: Vec<Layer<PD>>,
    pub history: Vec<HistoryEntry<PD>>,
    pub anchors: Vec<Anchor<PD>>,
    /// Note that these components are not yet parsed or checked for infinite loops. You need to
    /// call either ``GlifComponent::to_component_of`` on each of these, or ``Glif::flatten``.
    pub components: GlifComponents,
    pub flattened: Option<Outline<PD>>, // holds cached flattened glif (components to points)
    pub component_rects: Option<Vec<ComponentRect>>, // holds cached flattened component rects
    pub guidelines: Vec<Guideline<PD>>,
    pub width: Option<u64>,
    pub unicode: Vec<char>,
    pub name: String,
    /// This is an arbitrary glyph comment, exactly like the comment field in FontForge SFD.
    pub note: Option<String>,
    /// It's up to the API consumer to set this.
    #[cfg_attr(feature = "glifserde", serde(skip_serializing))]
    pub filename: Option<stdpath::PathBuf>,
}

pub struct MFEKCubicGlif<PD: PointData>(MFEKGlif<PD>);

impl<PD: PointData> From<Glif<PD>> for MFEKGlif<PD> {
    fn from(glif: Glif<PD>) -> Self {
        let mut layers = Vec::new();
        let history = Vec::new();

        let mut ret = MFEKGlif {
            layers: vec![],
            history,
            flattened: None,
            component_rects: None,
            anchors: glif.anchors,
            components: glif.components,
            guidelines: glif.guidelines,
            width: glif.width,
            unicode: glif.unicode,
            name: glif.name,
            note: glif.note,
            filename: glif.filename,
        };

        layers.push(Layer {
            // Warning: due to Rust language limitations, the const
            // `layer::DEFAULT_LAYER_FORMAT_STR` is not usable here. Thus, the macro.
            name: format!(DEFAULT_LAYER_FORMAT_STR!(), 0),
            visible: true,
            color: None,
            outline: glif.outline.unwrap_or(Vec::new()).iter().map(|contour| contour.into() ).collect(),
            operation: None,
            images: glif.images.iter().map(|im| {
                let temp_affine: Affine = im.matrix().into();
                (im.clone(), temp_affine)
            }).collect(),
        });
        ret.layers = layers;

        ret
    }
}

impl<PD: PointData> From<MFEKGlif<PD>> for Glif<PD> {
    fn from(glif: MFEKGlif<PD>) -> Self {
        let outline = glif.layers[0].outline.iter().map(|contour| 
            match &contour.inner() {
                MFEKContourInner::Cubic(cubic) => cubic.clone(),
                // TODO: BETTER HANDLING!
                _ => panic!("Tried to convert non-cubic MFEKGlif to glif")
            }
            
        ).collect();

        let images = glif.layers[0].images.iter().map(|tupes| {
            tupes.0.clone()
        }).collect();

        Glif {
            anchors: glif.anchors.clone(),
            components: glif.components.clone(),
            guidelines: glif.guidelines.clone(),
            width: glif.width,
            unicode: glif.unicode.clone(),
            name: glif.name.clone(),
            filename: glif.filename.clone(),
            outline: Some(outline),
            images,
            note: glif.note.clone(),
            ..Glif::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry<PD: PointData> {
    pub description: String,
    pub layer_idx: Option<usize>,
    pub contour_idx: Option<usize>,
    pub point_idx: Option<usize>,
    pub guidelines: Vec<Guideline<PD>>, // UFO-level, not glyph-level which'd be in HistoryEntry::glyph
    pub selected: Option<HashSet<(usize, usize)>>,
    pub glyph: MFEKGlif<PD>,
}

pub type MFEKOutline<PD> = Vec<MFEKContour<PD>>;

#[cfg(feature = "skia")]
impl<PD: PointData> ToSkiaPaths for MFEKOutline<PD> {
    fn to_skia_paths(&self, spt: Option<SkiaPointTransforms>) -> SkiaPaths {
        let mut ret = SkiaPaths::default();

        let mut open = Path::new();
        let mut closed = Path::new();

        for contour in self {
            match &contour.inner() {
                MFEKContourInner::Cubic(cubic) => {
                    let firstpoint: &Point<PD> = match cubic.first() {
                        Some(p) => p,
                        None => { continue } // contour has no points
                    };
                    let skpath = cubic.to_skia_path(spt).unwrap(); // therefore we know it'll be Some
                    if firstpoint.ptype == PointType::Move {
                        &mut open
                    } else {
                        &mut closed
                    }.add_path(&skpath, (0., 0.), skia::path::AddPathMode::Append);
                },
                _ => { panic!("Attempted to to_skia_paths a mixed contour!") }
            }
        }

        if open.count_points() > 0 {
            ret.open = Some(open);
        }

        if closed.count_points() > 0 {
            ret.closed = Some(closed);
        }

        ret
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerOperation {
    Difference,
    Union,
    XOR,
    Intersect,
}

use self::inner::MFEKContourInner;

use super::GlifLike;

impl<PD: PointData> GlifLike for MFEKGlif<PD> {
    fn filename(&self) -> &Option<stdpath::PathBuf> {
        &self.filename
    }
    fn name(&self) -> &String {
        &self.name
    }
}

#[test]
fn test_tostring() {
    use crate::CapType;
    assert!(format!("{}", CapType::Circle) == CapType::Circle.to_string() && CapType::Circle.to_string() == String::from("circle"));
}
