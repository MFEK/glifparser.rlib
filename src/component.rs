//! .glif `<component>`

mod xml;

use crate::error::GlifParserError;
#[cfg(feature = "mfek")]
use crate::glif::mfek::MFEKGlif;
use crate::glif::{self, Glif};
use crate::matrix::GlifMatrix;
use crate::outline::Outline;
use crate::point::PointData;

use integer_or_float::IntegerOrFloat;
use kurbo::Affine;
pub use trees::{Forest, Node, Tree};
use IntegerOrFloat::Float;

#[cfg(feature = "glifserde")]
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[allow(non_snake_case)] // to match UFO spec https://unifiedfontobject.org/versions/ufo3/glyphs/glif/#component
#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct GlifComponent {
    pub base: String,
    pub filename: Option<PathBuf>,
    pub xScale: IntegerOrFloat,
    pub xyScale: IntegerOrFloat,
    pub yxScale: IntegerOrFloat,
    pub yScale: IntegerOrFloat,
    pub xOffset: IntegerOrFloat,
    pub yOffset: IntegerOrFloat,
    pub identifier: Option<String>,
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
/// A container meant for yelding a [`Forest<Component<PD>>`].
///
/// Please see [`impl From<GlifComponents> for Result<Forest<Component<PD>>,
/// GlifParserError>`](#conversion).
pub struct GlifComponents {
    pub root: String,
    pub vec: Vec<GlifComponent>,
    pub uniques: HashSet<String>
}

impl GlifComponents {
    pub fn new() -> Self {
        Default::default()
    }
}

impl GlifComponent {
    pub fn new() -> Self {
        Self {
            xScale: Float(1.0),
            yScale: Float(1.0),
            ..Default::default()
        }
    }
}

impl GlifComponent {
    pub fn matrix(&self) -> GlifMatrix {
        GlifMatrix(
            self.xScale,
            self.xyScale,
            self.yxScale,
            self.yScale,
            self.xOffset,
            self.yOffset,
        )
    }
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Component<PD: PointData> {
    pub glif: Glif<PD>,
    pub matrix: Affine,
}

impl<PD: PointData> Component<PD> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentRect {
    pub minx: f32,
    pub miny: f32,
    pub maxx: f32,
    pub maxy: f32,
    pub name: String,
}

impl ComponentRect {
    pub fn from_rect_and_name(minx: f32, miny: f32, maxx: f32, maxy: f32, name: String) -> Self {
        Self {
            minx,
            miny,
            maxx,
            maxy,
            name,
        }
    }
}

use std::fs;
impl GlifComponent {
    /// Sets the filename of a component relative to its base's filename (`gliffn`)
    pub fn set_file_name<F: AsRef<Path>>(&mut self, gliffn: F) {
        let mut retglifname = gliffn.as_ref().to_path_buf();
        retglifname.set_file_name(glif::name_to_filename(&self.base, true));
        self.filename = Some(retglifname);
    }

    /// Must have filename set, and that file must be readable, for this to work.
    pub fn to_component<PD: PointData>(&self) -> Result<Component<PD>, GlifParserError> {
        let gliffn = &self
            .filename
            .as_ref()
            .ok_or(GlifParserError::GlifFilenameNotSet(self.base.clone()))?;

        let mut ret = Component::new();
        ret.matrix = self.matrix().into();
        ret.glif.name = self.base.clone();
        ret.glif.filename = self.filename.clone();
        let component_xml = fs::read_to_string(&gliffn).or(Err(GlifParserError::GlifFilenameNotSet(
            "Glif filename leads to unreadable file".to_string(),
        )))?;
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

pub trait FlattenedGlif
where
    Self: Clone,
{
    /// Check that all components in your .glif file really resolve, and if they do, get their
    /// contours and apply their matrices. If you want bounding rectangles as this process is done
    /// with a logical name for each rectangle you can draw, pass in `rects` as a `&mut`
    fn flattened(&self, rects: &mut Option<Vec<ComponentRect>>) -> Result<Self, GlifParserError>;
}

fn apply_component_rect<PD: PointData>(
    last: &Node<Component<PD>>,
    minx: &mut f32,
    miny: &mut f32,
    maxx: &mut f32,
    maxy: &mut f32,
    final_outline: &mut Outline<PD>,
) {
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
                    if p.x < *minx || is_first {
                        *minx = p.x;
                    }
                    if p.y < *miny || is_first {
                        *miny = p.y;
                    }
                    if p.x > *maxx || is_first {
                        *maxx = p.x;
                    }
                    if p.y > *maxy || is_first {
                        *maxy = p.y;
                    }

                    for m in &matrices {
                        p.apply_matrix(*m);
                    }

                    to_transform[i][j] = p;
                }
            }
            final_outline.extend(to_transform);
        }
        None => {}
    }
}

#[rustfmt::skip]
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
                let components_r: Result<Forest<Component<PD>>, _> = (ret.components).into();
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

/// # Conversion
///
/// This impl builds up a forest of trees for a glyph's components. Imagine a hungarumlaut (˝).
///
/// This character may be built of glyph components, as such:
///
/// ```plain
/// hungarumlaut
///    /    \
///   /      \
/// grave  grave
///   |      |
/// acute  acute
/// ```
///
/// This function will give you a Forest of both of the sub-trees. ([`Forest<Component>`]). The elements
/// of a [`Forest`] are [`Tree<Component>`]. For safety reasons, this function cannot always return a
/// [`Forest`]. Sometimes, .glif files can be malformed, containing components which refer to
/// themselves or to components higher up the tree.
impl<PD: PointData> From<GlifComponents> for Result<Forest<Component<PD>>, GlifParserError> {
    fn from(mut glifcs: GlifComponents) -> Self {
        let mut forest = Forest::new();
        let components: Vec<_> = glifcs.vec.drain(..).collect();

        let cs  = components.into_iter().map(|c| {
            glifcs.build_component_tree(c.to_component()?)
        }).collect::<Result<Vec<_>, GlifParserError>>()?;

        for c in cs {
            forest.push_back(c);
        }

        Ok(forest)
    }
}

/// Builds a tree of components in a recursive manner. It takes a mutable reference to a `HashSet`
/// called `uniques`, which keeps track of the unique component names seen so far in the current
/// subtree. This prevents cycles from occurring.
///
/// If a loop is found in the tree (for example, gershayim refers to grave refers to grave), an
/// error is returned with the appropriate message. The error type is
/// [`GlifParserError::GlifComponentsCyclical`], which takes a formatted string to provide more
/// details about the error.
impl GlifComponents {
    /// You should not need to call this function directly; see [§ Conversion](#conversion).
    pub fn build_component_tree<PD: PointData>(
        &mut self,
        component: Component<PD>,
    ) -> Result<Tree<Component<PD>>, GlifParserError> {
        let mut tree = Tree::new(component.clone());

        for gc in component.glif.components.vec.into_iter() {
            if !self.uniques.insert(gc.base.clone()) {
                return Err(GlifParserError::GlifComponentsCyclical(format!(
                    "in glif {}, {} refers to {} (trying to flatten {})",
                    gc.base, component.glif.name, gc.base, self.root
                )));
            }

            tree.push_back(self.build_component_tree(gc.to_component()?)?);
            self.uniques.remove(&gc.base);
        }

        Ok(tree)
    }
}
