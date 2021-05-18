use std::collections::HashSet;
use std::path;

use skia_safe as skia;
use skia::{Matrix as SkMatrix, Path};
use kurbo::Affine;
use serde::{Serialize, Deserialize};

use crate::{PointType, outline::skia::{SkiaPaths, SkiaPointTransforms, ToSkiaPath, ToSkiaPaths}, point::Point, image::GlifImage};

use crate::{
    outline::OutlineType, point::PointData, Anchor, ComponentRect, Glif, component::GlifComponents, Guideline,
    Outline,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFEKPointData;

// This is an intermediary form used in MFEKglif and other tools. You can .into() a glif into this
// make changes to MFEK data and then turn it back into a standard UFO glif before saving.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MFEKGlif<PD: PointData> {
    pub layers: Vec<Layer<PD>>,
    pub history: Vec<HistoryEntry<PD>>,
    pub order: OutlineType,
    pub anchors: Vec<Anchor>,
    /// Note that these components are not yet parsed or checked for infinite loops. You need to
    /// call either ``GlifComponent::to_component_of`` on each of these, or ``Glif::flatten``.
    pub components: GlifComponents,
    pub flattened: Option<Outline<PD>>, // holds cached flattened glif (components to points)
    pub component_rects: Option<Vec<ComponentRect>>, // holds cached flattened component rects
    pub guidelines: Vec<Guideline>,
    pub width: Option<u64>,
    pub unicode: Vec<char>,
    pub name: String,
    pub format: u8, // we only understand 2
    /// It's up to the API consumer to set this.
    pub filename: Option<path::PathBuf>,
}

impl From<Glif<MFEKPointData>> for MFEKGlif<MFEKPointData> {
    fn from(glif: Glif<MFEKPointData>) -> Self {
        if let Some(mfek_lib) = glif.private_lib {
            return serde_json::from_str(mfek_lib.as_str()).unwrap();
        } else {
            let mut layers = Vec::new();
            let history = Vec::new();

            let mut ret = MFEKGlif {
                layers: vec![],
                history: history,
                flattened: None,
                component_rects: None,
                order: glif.order,
                anchors: glif.anchors,
                components: glif.components,
                guidelines: glif.guidelines,
                width: glif.width,
                unicode: glif.unicode,
                name: glif.name,
                format: glif.format,
                filename: glif.filename,
            };

            use crate::matrix::skia::ToSkiaMatrix;
            layers.push(Layer {
                name: "Layer 0".to_string(),
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
            format: glif.format,
            filename: glif.filename.clone(),
            outline: Some(outline),
            images: images,
            note: None,
            lib: None,
            private_lib: Some(serde_json::to_string(&glif).unwrap()),
            private_lib_root: "MFEK",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HistoryType {
    LayerModified,
    LayerAdded,
    LayerDeleted,
    LayerMoved{
        to: usize,
        from: usize,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry<PD: PointData> {
    pub description: String,
    pub layer_idx: Option<usize>,
    pub contour_idx: Option<usize>,
    pub point_idx: Option<usize>,
    pub selected: Option<HashSet<(usize, usize)>>,
    pub layer: Layer<PD>,
    pub kind: HistoryType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MFEKContour<PD: PointData> {
    pub inner: Vec<Point<PD>>,
    pub operation: Option<ContourOperations>,
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
            inner: contour.clone(),
            operation: None,
        }
    }
}


pub type MFEKOutline<PD: PointData> = Vec<MFEKContour<PD>>;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer<PD: PointData> {
    pub name: String,
    pub visible: bool,
    pub color: Option<[f32; 4]>,
    pub outline: MFEKOutline<PD>,
    pub operation: Option<LayerOperation>,
    pub images: Vec<(GlifImage, Affine)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContourOperations {
    VariableWidthStroke { data: VWSContour },
    PatternAlongPath { data: PAPContour }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PatternCopies {
    Single,
    Repeated,
    Fixed(usize) // TODO: Implement
}

// pff - no splitting
// simple - split each curve at it's midpoint
// angle - split the input pattern each x degrees in change in direction on the path
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PatternSubdivide {
    Off,
    Simple(usize), // The value here is how many times we'll subdivide simply
    //Angle(f64) TODO: Implement.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PatternHandleDiscontinuity {
    Off, // no handling
    Split(f64) 
    // Cut TODO: implement
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PAPContour {
    pub pattern: MFEKOutline<MFEKPointData>,
    pub copies: PatternCopies,
    pub subdivide: PatternSubdivide,
    pub is_vertical: bool, // TODO: Implement this. Might replace it with a general rotation parameter to make it more useful.
    pub stretch: bool,
    pub spacing: f64,
    pub simplify: bool,
    pub normal_offset: f64,
    pub tangent_offset: f64,
    pub pattern_scale: (f64, f64),
    pub center_pattern: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VWSContour {
    pub handles: Vec<VWSHandle>,
    pub join_type: JoinType,
    pub cap_start_type: CapType,
    pub cap_end_type: CapType,
    pub remove_internal: bool,
    pub remove_external: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum InterpolationType {
    Linear,
    Null,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CapType {
    Round,
    Square,
    Circle,
    Custom,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LayerOperation {
    Difference,
    Union,
    XOR,
    Intersect,
}

use super::GlifLike;

impl<PD: PointData> GlifLike for MFEKGlif<PD> {
    fn filename(&self) -> &Option<path::PathBuf> {
        &self.filename
    }
    fn name(&self) -> &String {
        &self.name
    }
}
