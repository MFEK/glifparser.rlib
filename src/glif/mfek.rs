use std::collections::HashSet;
use std::path as stdpath;
use std::{fmt::Display, str::FromStr};

use kurbo::Affine;
use serde::{Deserialize, Serialize};
#[cfg(feature = "skia")]
use skia_safe::{self as skia, Path};

use crate::anchor::Anchor;
use crate::component::{ComponentRect, GlifComponents};
use crate::error::{mfek::*, GlifParserError};
use crate::glif::Glif;
use crate::guideline::Guideline;
#[cfg(feature = "skia")]
use crate::outline::skia::{SkiaPaths, SkiaPointTransforms, ToSkiaPath, ToSkiaPaths};
use crate::outline::{Contour, Outline, OutlineType};
use crate::point::{Point, PointData, PointType};

#[macro_use] pub mod layer;
pub use layer::Layer;
pub(crate) use DEFAULT_LAYER_FORMAT_STR;
pub mod traits;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MFEKPointData;
impl PointData for MFEKPointData {}
impl PointData for Option<MFEKPointData> {}
impl From<()> for MFEKPointData {
    fn from(_: ()) -> Self {
        Self::default()
    }
}
impl Into<()> for MFEKPointData {
    fn into(self) -> () {
        ()
    }
}

/// This is an intermediary form used in MFEKglif and other tools. You can .into() a glif into this
/// make changes to MFEK data and then turn it back into a standard UFO glif before saving.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MFEKGlif<PD: PointData> {
    pub layers: Vec<Layer<PD>>,
    pub history: Vec<HistoryEntry<PD>>,
    pub order: OutlineType,
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
    pub filename: Option<stdpath::PathBuf>,
}

impl<PD: PointData> From<Glif<PD>> for MFEKGlif<PD> {
    fn from(glif: Glif<PD>) -> Self {
        let mut layers = Vec::new();
        let history = Vec::new();

        let mut ret = MFEKGlif {
            layers: vec![],
            history,
            flattened: None,
            component_rects: None,
            order: glif.order,
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
            contour.inner.clone()
        ).collect();

        let images = glif.layers[0].images.iter().map(|tupes| {
            tupes.0.clone()
        }).collect();

        Glif {
            order: glif.order,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MFEKContour<PD: PointData> {
    pub inner: Vec<Point<PD>>,
    pub operation: Option<ContourOperations<PD>>,
}

impl<PD: PointData> From<&Vec<Point<PD>>> for MFEKContour<PD> {
    fn from(contour: &Vec<Point<PD>>) -> Self {
        Self {
            inner: contour.clone(),
            operation: None,
        }
    }
}

impl<PD: PointData> From<Vec<Point<PD>>> for MFEKContour<PD> {
    fn from(contour: Vec<Point<PD>>) -> Self {
        Self {
            inner: contour,
            operation: None,
        }
    }
}

impl<PD: PointData> Into<Contour<PD>> for MFEKContour<PD> {
    fn into(self) -> Contour<PD> {
        self.inner
    }
}

pub type MFEKOutline<PD> = Vec<MFEKContour<PD>>;

#[cfg(feature = "skia")]
impl<PD: PointData> ToSkiaPaths for MFEKOutline<PD> {
    fn to_skia_paths(&self, spt: Option<SkiaPointTransforms>) -> SkiaPaths {
        let mut ret = SkiaPaths {
            open: None,
            closed: None
        };

        let mut open = Path::new();
        let mut closed = Path::new();

        for contour in self {
            let firstpoint: &Point<PD> = match contour.inner.first() {
                Some(p) => p,
                None => { continue } // contour has no points
            };
            let skpath = contour.inner.to_skia_path(spt).unwrap(); // therefore we know it'll be Some
            if firstpoint.ptype == PointType::Move {
                &mut open
            } else {
                &mut closed
            }.add_path(&skpath, (0., 0.), skia::path::AddPathMode::Append);
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

// The reason that all this is here and not in MFEK/math.rlib is because this data needs to be
// serialized and deserialized in glif files. So, MFEK/math.rlib gets its structs from here.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ContourOperations<PD: PointData> {
    VariableWidthStroke { data: VWSContour },
    PatternAlongPath { data: PAPContour<PD> },
    DashAlongPath { data: DashContour },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternCopies {
    Single,
    Repeated,
    Fixed(usize) // TODO: Implement
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternSubdivide {
    /// no splitting
    Off,
    /// split each curve at its midpoint
    Simple(usize), // The value here is how many times we'll subdivide simply
    // split the input pattern each x degrees in change in direction on the path
    //Angle(f64) TODO: Implement.
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternHandleDiscontinuity {
    /// no handling
    Off,
    /// handle by splitting
    Split(f64) 
    // Cut TODO: implement
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PatternStretch {
    /// no stretching
    Off,
    /// stretch the pattern
    On,
    /// stretch the spacing between the pattern
    Spacing,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PAPContour<PD: PointData> {
    pub pattern: MFEKOutline<PD>,
    pub copies: PatternCopies,
    pub subdivide: PatternSubdivide,
    pub is_vertical: bool, // TODO: Implement this. Might replace it with a general rotation parameter to make it more useful.
    pub stretch: PatternStretch,
    pub spacing: f64,
    pub simplify: bool,
    pub normal_offset: f64,
    pub tangent_offset: f64,
    pub pattern_scale: (f64, f64),
    pub center_pattern: bool,
    pub prevent_overdraw: f64,
    pub two_pass_culling: bool,
    pub reverse_path: bool,
    pub reverse_culling: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VWSContour {
    pub handles: Vec<VWSHandle>,
    pub join_type: JoinType,
    pub cap_start_type: CapType,
    pub cap_end_type: CapType,
    pub remove_internal: bool,
    pub remove_external: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DashCull {
    pub width: f32,
    pub area_cutoff: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DashContour {
    pub stroke_width: f32,
    pub cull: Option<DashCull>,
    pub dash_desc: Vec<f32>,
    pub include_last_path: bool,
    pub paint_join: u8, // skia::PaintJoin,
    pub paint_cap: u8, // skia::PaintCap,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum InterpolationType {
    Null,
    Linear,
}

impl Display for InterpolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            InterpolationType::Null => "none",
            InterpolationType::Linear => "linear",
        })
    }
}

impl FromStr for InterpolationType {
    type Err = GlifParserError/*::TypeConversionError(type, s)*/;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(InterpolationType::Null),
            "linear" => Ok(InterpolationType::Linear),
            _ => Err(GlifParserError::TypeConversionError{req_type: "InterpolationType", req_variant: s.to_owned()}),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct VWSHandle {
    pub left_offset: f64,
    pub right_offset: f64,
    pub tangent_offset: f64,
    pub interpolation: InterpolationType,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum JoinType {
    Bevel,
    Miter,
    Circle,
    Round,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            JoinType::Bevel => "bevel",
            JoinType::Miter => "miter",
            JoinType::Circle => "circle",
            JoinType::Round => "round",
        })
    }
}

impl FromStr for JoinType {
    type Err = GlifParserError/*::TypeConversionError(type, s)*/;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bevel" => Ok(JoinType::Bevel),
            "miter" => Ok(JoinType::Miter),
            "circle" => Ok(JoinType::Circle),
            "round" => Ok(JoinType::Round),
            _ => Err(GlifParserError::TypeConversionError{req_type: "JoinType", req_variant: s.to_owned()}),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CapType {
    Custom,
    Square,
    Circle,
    Round,
}

impl Display for CapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            CapType::Custom => "custom",
            CapType::Square => "square",
            CapType::Circle => "circle",
            CapType::Round => "round",
        })
    }
}

impl FromStr for CapType {
    type Err = GlifParserError/*::TypeConversionError(type, s)*/;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "custom" => Ok(CapType::Custom),
            "square" => Ok(CapType::Square),
            "circle" => Ok(CapType::Circle),
            "round" => Ok(CapType::Round),
            _ => {
                if s.ends_with(".glif") {
                    Ok(CapType::Custom)
                } else {
                    Err(GlifParserError::TypeConversionError{req_type: "CapType", req_variant: s.to_owned()})
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerOperation {
    Difference,
    Union,
    XOR,
    Intersect,
}

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
  assert!(format!("{}", CapType::Circle) == CapType::Circle.to_string() && CapType::Circle.to_string() == String::from("circle"));
}
