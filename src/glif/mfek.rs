use std::collections::{HashMap, HashSet};
use std::path;

use crate::{
    outline::OutlineType, point::PointData, Anchor, ComponentRect, Glif, component::GlifComponents, Guideline,
    Outline,
};

#[derive(Debug, Clone)]
pub struct MFEKPointData;

// This is an intermediary form used in MFEKglif and other tools. You can .into() a glif into this
// make changes to MFEK data and then turn it back into a standard UFO glif before saving.
#[derive(Clone, Debug)]
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
    /// We give you the <lib> as an XML Element. Note, however, that in the UFO spec it is a plist
    /// dictionary. You're going to need to parse this with a plist parser, such as plist.rs. You
    /// may want to tell xmltree to write it back to a string first; however, it may be possible to
    /// parse plist from xmltree::Element. Might change some day to a ``plist::Dictionary``.
    pub lib: Option<xmltree::Element>,
}

impl<PD: PointData> From<Glif<PD>> for MFEKGlif<PD> {
    fn from(glif: Glif<PD>) -> Self {
        if let Some(_mfek_lib) = glif.private_lib {
            todo!("Actually load private lib.")
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
                lib: glif.lib,
            };

            layers.push(Layer {
                outline: glif.outline,
                contour_ops: HashMap::new(),
                operation: None,
            });
            ret.layers = layers;

            ret
        }
    }
}

#[derive(Clone, Debug)]
pub enum HistoryType {
    LayerModified,
    LayerAdded,
    LayerDeleted,
    LayerMoved{
        to: usize,
        from: usize,
    }
}

#[derive(Clone, Debug)]
pub struct HistoryEntry<PD: PointData> {
    pub description: String,
    pub layer_idx: Option<usize>,
    pub contour_idx: Option<usize>,
    pub point_idx: Option<usize>,
    pub selected: Option<HashSet<(usize, usize)>>,
    pub layer: Layer<PD>,
    pub kind: HistoryType,
}

#[derive(Clone, Debug)]
pub struct Layer<PD: PointData> {
    pub outline: Option<Outline<PD>>,
    pub contour_ops: HashMap<usize, ContourOp>,
    pub operation: Option<LayerOperation>,
}

#[derive(Clone, Debug)]
pub enum ContourOp {
    VariableWidthStroke { contour: VWSContour },
}
#[derive(Debug, Clone)]
pub struct VWSContour {
    pub id: usize,
    pub handles: Vec<VWSHandle>,
    pub join_type: JoinType,
    pub cap_start_type: CapType,
    pub cap_end_type: CapType,
    pub remove_internal: bool,
    pub remove_external: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum InterpolationType {
    Linear,
    Null,
}

#[derive(Debug, Clone, Copy)]
pub struct VWSHandle {
    pub left_offset: f64,
    pub right_offset: f64,
    pub tangent_offset: f64,
    pub interpolation: InterpolationType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinType {
    Bevel,
    Miter,
    Circle,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapType {
    Round,
    Square,
    Circle,
    Custom,
}

#[derive(Clone, Debug)]
pub enum LayerOperation {
    Difference,
    Union,
    XOR,
    Intersect,
}
