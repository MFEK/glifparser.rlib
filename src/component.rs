use crate::error::GlifParserError;
use crate::glif::{self, Glif};
#[cfg(feature = "mfek")]
use crate::glif::mfek::MFEKGlif;
use crate::matrix::GlifMatrix;
use crate::point::{Handle, PointData, WhichHandle};
use crate::outline::Outline;

use integer_or_float::IntegerOrFloat;
use kurbo::Affine;
use trees::{Forest, Tree, Node};

use std::path::{Path, PathBuf};

#[allow(non_snake_case)] // to match UFO spec https://unifiedfontobject.org/versions/ufo3/glyphs/glif/#component
#[derive(Clone, Debug, PartialEq)]
pub struct GlifComponent {
    pub base: String,
    pub filename: Option<PathBuf>,
    pub xScale: IntegerOrFloat,
    pub xyScale: IntegerOrFloat,
    pub yxScale: IntegerOrFloat,
    pub yScale: IntegerOrFloat,
    pub xOffset: IntegerOrFloat,
    pub yOffset: IntegerOrFloat,
    pub identifier: Option<String>
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlifComponents {
    pub root: String,
    pub vec: Vec<GlifComponent>,
}

impl GlifComponents {
    pub fn new() -> Self {
        Self {
            root: String::new(),
            vec: vec![]
        }
    }
}

impl GlifComponent {
    pub fn new() -> Self {
        Self {
            base: String::new(),
            filename: None,
            xScale: IntegerOrFloat::Integer(1),
            xyScale: IntegerOrFloat::Integer(0),
            yxScale: IntegerOrFloat::Integer(0),
            yScale: IntegerOrFloat::Integer(1),
            xOffset: IntegerOrFloat::Integer(0),
            yOffset: IntegerOrFloat::Integer(0),
            identifier: None
        }
    }
}

impl GlifComponent {
    pub fn matrix(&self) -> GlifMatrix {
        GlifMatrix(self.xScale, self.xyScale, self.yxScale, self.yScale, self.xOffset, self.yOffset)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Component<PD: PointData> {
    pub glif: Glif<PD>,
    pub matrix: Affine
}

impl<PD: PointData> Component<PD> {
    pub fn new() -> Self {
        Component {
            glif: Glif::new(),
            matrix: Affine::IDENTITY
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentRect {
    pub minx: f32,
    pub miny: f32,
    pub maxx: f32,
    pub maxy: f32,
    pub name: String,
}

impl ComponentRect {
    pub fn from_rect_and_name(minx: f32, miny: f32, maxx: f32, maxy: f32, name: String) -> Self {
        Self { minx, miny, maxx, maxy, name }
    }
}

use std::fs;
impl GlifComponent {
    pub fn set_file_name<F: AsRef<Path>>(&mut self, gliffn: F) {
        let mut retglifname = gliffn.as_ref().to_path_buf();
        retglifname.set_file_name(glif::name_to_filename(&self.base));
        self.filename = Some(retglifname);
    }

    /// Must have filename set, and that file must be readable, for this to work.
    pub fn to_component<PD: PointData>(&self) -> Result<Component<PD>, GlifParserError> {
        let gliffn = &self.filename.as_ref().ok_or(GlifParserError::GlifFilenameNotSet(self.base.clone()))?;

        let mut ret = Component::new();
        ret.matrix = self.matrix().into();
        ret.glif.name = self.base.clone();
        ret.glif.filename = self.filename.clone();
        let component_xml = fs::read_to_string(&gliffn).unwrap();
        let mut newglif: Glif<PD> = glif::read(&component_xml)?;
        for component in newglif.components.vec.iter_mut() {
            component.set_file_name(&gliffn);
        }
        ret.glif.components = newglif.components;
        ret.glif.anchors = newglif.anchors;
        ret.glif.outline = newglif.outline;
        Ok(ret)
    }

    pub fn refers_to<PD: PointData>(&self, glif: &Glif<PD>) -> bool {
        self.base == glif.name
    }
}

pub trait FlattenedGlif where Self: Clone {
    fn flattened(&self, rects: &mut Option<Vec<ComponentRect>>) -> Result<Self, GlifParserError>;
}

fn apply_component_rect<PD: PointData>(last: &Node<Component<PD>>, minx: &mut f32, miny: &mut f32, maxx: &mut f32, maxy: &mut f32, final_outline: &mut Outline<PD>) {
    let mut matrices = vec![];
    matrices.push((*last).data().matrix);

    // Climb the tree, building a Vec of matrices for this component
    let mut pt = last.parent();
    while let Some(parent) = pt {
        matrices.push(parent.data().matrix);
        pt = parent.parent();
    }

    match (*last).data().glif.outline {
        Some(ref o) => {
            let mut to_transform = o.clone();
            for i in 0..to_transform.len() {
                for j in 0..to_transform[i].len() {
                    let is_first = i == 0 && j == 0;
                    let mut p = to_transform[i][j].clone();
                    let kbp = matrices.iter().fold(KurboPoint::new(p.x as f64, p.y as f64), |p, m| *m * p);
                    p.x = kbp.x as f32;
                    p.y = kbp.y as f32;
                    if p.x < *minx || is_first { *minx = p.x; }
                    if p.y < *miny || is_first { *miny = p.y; }
                    if p.x > *maxx || is_first { *maxx = p.x; }
                    if p.y > *maxy || is_first { *maxy = p.y; }

                    if p.a != Handle::Colocated {
                        let (ax, ay) = p.handle_or_colocated(WhichHandle::A, |f|f, |f|f);
                        let kbpa = matrices.iter().fold(KurboPoint::new(ax as f64, ay as f64), |p, m| *m * p);
                        p.a = Handle::At(kbpa.x as f32, kbpa.y as f32);
                    }

                    if p.b != Handle::Colocated {
                        let (bx, by) = p.handle_or_colocated(WhichHandle::B, |f|f, |f|f);
                        let kbpb = matrices.iter().fold(KurboPoint::new(bx as f64, by as f64), |p, m| *m * p);
                        p.b = Handle::At(kbpb.x as f32, kbpb.y as f32);
                    }

                    to_transform[i][j] = p;
                }
            }
            final_outline.extend(to_transform);
        },
        None => {}
    }
}


use kurbo::Point as KurboPoint;
macro_rules! impl_flattened_glif {
    ($glifstruct:ident, $outline:ident) => { 

        impl<PD: PointData> FlattenedGlif for $glifstruct<PD> {
            /// Flatten a UFO .glif with components.
            ///
            /// Can fail if the .glif's components form an infinite loop.
            // How this works is we start at the bottom of the tree, take all of the Affine matrices which
            // describe the transformation of the glyph's points, and continuously apply them until we run
            // out of nodes of the tree. Finally, we set our outline to be the final transformed outline,
            // and consider ourselves as no longer being made up of components.
            fn flattened(&self, rects: &mut Option<Vec<ComponentRect>>) -> Result<Self, GlifParserError> {
                let mut ret = self.clone();
                let components_r: Result<Forest<Component<PD>>, _> = (&ret.components).into();
                let components = components_r?;
                let mut final_outline: Outline<PD> = Outline::new();
                let mut component_rects = vec![];

                for mut component in components {
                    let (mut minx, mut miny, mut maxx, mut maxy) = (0., 0., 0., 0.);
                    // This unwrap is safe because at this point we know we haven't yet exhausted the
                    // Forest<Component>.
                    let component_name = component.data().glif.name.clone();
                    apply_component_rect(&component, &mut minx, &mut miny, &mut maxx, &mut maxy, &mut final_outline);
                    while let Some(last) = component.back() {
                        apply_component_rect(&last, &mut minx, &mut miny, &mut maxx, &mut maxy, &mut final_outline);
                        component.pop_back();
                    }
                    component_rects.push(ComponentRect { minx, maxx, miny, maxy, name: component_name });
                }

                ret.$outline = Some(final_outline);

                // If we were to leave this here, then API consumers would potentially draw component outlines on top of components.
                ret.components = GlifComponents::new();

                if let Some(ptr) = rects {
                    *ptr = component_rects;
                }

                Ok(ret)
            }
        }

    }
}
impl_flattened_glif!(Glif, outline);
#[cfg(feature = "mfek")]
impl_flattened_glif!(MFEKGlif, flattened);

// This impl builds up a forest of trees for a glyph's components. Imagine a hungarumlaut (˝).
//
// This character may be built of glyph components, as such:
//
// hungarumlaut
//    /    \
//   /      \
// grave  grave
//   |      | 
// acute  acute
//
// This function will give you a Forest of both of the sub-trees. (Forest<Component>). The elements
// of a Forest are Tree<Component>. For safety reasons, this function cannot always return a
// Forest, however. Sometimes, .glif files can be malformed, containing components which refer to
// themselves, or to components higher up the tree. Therefore, the inner recursive function
// `component_to_tree` receives a Tree of `uniques`, calculated for each sub-tree, and also a global
// mutable `unique_found` flag, for the entire Forest.
//
// If a loop is found in the tree (for example, grave refers to grave), `unique_found` is set,
// poisoning the function, returning an error. unique_found is (String, String) for error formatting;
// however, should be considered basically equivalent to a boolean.
impl<PD: PointData> From<&GlifComponents> for Result<Forest<Component<PD>>, GlifParserError> {
    fn from(glifcs: &GlifComponents) -> Self {
        let mut unique_found = None;

        fn component_to_tree<PD: PointData>(component: Component<PD>, uniques: &mut Tree<String>, unique_found: &mut Option<(String, String)>) -> Result<Tree<Component<PD>>, GlifParserError> {
            let mut tree = Tree::new(component.clone());
            for gc in component.glif.components.vec.iter() {
                let component_inner = gc.to_component()?;
                uniques.back_mut().unwrap().push_back(Tree::new(gc.base.clone()));
                // Generate a list of parents for the backmost node
                let parents = match uniques.back() {
                    Some(mut node) => {
                        while let Some(nn) = node.back() {
                            node = nn;
                        };
                        let mut parents: Vec<String> = vec![];
                        while let Some(p) = node.parent() {
                            parents.push(p.data().clone());
                            node = p;
                        }
                        parents
                    },
                    None => vec![]
                };
                if parents.contains(&gc.base) || gc.base == component.glif.name {
                    return {
                        *unique_found = Some((component.glif.name.clone(), gc.base.clone()));
                        Ok(tree)
                    }
                }
                tree.push_back(component_to_tree(component_inner, uniques, unique_found)?);
            }
            Ok(tree)
        }

        let mut forest = Forest::new();
        let cs: Vec<_> = glifcs.vec.iter().map(|gc| {
            let mut uniques: Tree<String> = Tree::new(glifcs.root.clone());
            uniques.push_back(Tree::new(gc.base.clone()));
            component_to_tree(gc.to_component()?, &mut uniques, &mut unique_found)
        }).collect();

        for c in cs {
            forest.push_back(c?);
        }

        match unique_found {
            Some((base, unique)) => {Err(GlifParserError::GlifComponentsCyclical(format!("in glif {}, {} refers to {}", &glifcs.root, base, unique)))},
            None => Ok(forest)
        }
    }
}
