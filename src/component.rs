use crate::error::GlifParserError;
use crate::glif::{self, Glif};
use crate::matrix::GlifMatrix;
use crate::point::{Handle, PointData, WhichHandle};
use crate::outline::Outline;

use integer_or_float::IntegerOrFloat;
use kurbo::Affine;

#[allow(non_snake_case)] // to match UFO spec https://unifiedfontobject.org/versions/ufo3/glyphs/glif/#component
#[derive(Clone, Debug, PartialEq)]
pub struct GlifComponent {
    pub base: String,
    pub xScale: IntegerOrFloat,
    pub xyScale: IntegerOrFloat,
    pub yxScale: IntegerOrFloat,
    pub yScale: IntegerOrFloat,
    pub xOffset: IntegerOrFloat,
    pub yOffset: IntegerOrFloat,
    pub identifier: Option<String>
}

impl GlifComponent {
    pub fn new() -> Self {
        Self {
            base: String::new(),
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

use std::fs;
impl GlifComponent {
    pub fn to_component_of<PD: PointData>(&self, glif: &Glif<PD>) -> Result<Component<PD>, GlifParserError> {
        let gliffn = &glif.filename.as_ref().ok_or(GlifParserError::GlifFilenameNotSet(glif.name.clone()))?;

        let mut ret = Component::new();
        ret.matrix = self.matrix().into();
        ret.glif.name = self.base.clone();
        let mut retglifname = gliffn.to_path_buf();
        retglifname.set_file_name(ret.glif.name_to_filename());
        let component_xml = fs::read_to_string(&retglifname).unwrap();
        ret.glif.filename = Some(retglifname);
        let newglif: Glif<PD> = glif::read(&component_xml)?;
        ret.glif.components = newglif.components;
        ret.glif.anchors = newglif.anchors;
        ret.glif.outline = newglif.outline;
        Ok(ret)
    }

    pub fn refers_to<PD: PointData>(&self, glif: &Glif<PD>) -> bool {
        self.base == glif.name
    }
}

use kurbo::Point as KurboPoint;
impl<PD: PointData> Glif<PD> {
    /// Flatten a UFO .glif with components.
    ///
    /// Can fail if the .glif's components form an infinite loop.
    // How this works is we start at the bottom of the tree, take all of the Affine matrices which
    // describe the transformation of the glyph's points, and continuously apply them until we run
    // out of nodes of the tree. Finally, we set our outline to be the final transformed outline,
    // and consider ourselves as no longer being made up of components.
    pub fn flatten(mut self) -> Result<Self, GlifParserError> {
        let components_r: Result<Forest<Component<PD>>, _> = (&self).into();
        let components = components_r?;
        let mut final_outline: Outline<PD> = Outline::new();

        for mut component in components {
            while let Some(last) = component.back_mut() {
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
                                let mut p = to_transform[i][j].clone();
                                let kbp = matrices.iter().fold(KurboPoint::new(p.x as f64, p.y as f64), |p, m| *m * p);
                                p.x = kbp.x as f32;
                                p.y = kbp.y as f32;

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

                component.pop_back();
            }
        }

        self.outline = Some(final_outline);

        // If we were to leave this here, then API consumers would potentially draw component outlines on top of components.
        self.components = vec![];

        Ok(self)
    }
}

use std::collections::HashSet;
use trees::{Forest, Tree};
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
// `component_to_tree` receives a Vec of `uniques`, calculated for each sub-tree, and also a global
// mutable `unique_found` flag, for the entire Forest.
//
// If a loop is found in the tree (for example, grave refers to grave), `unique_found` is set,
// poisoning the function, returning an error. unique_found is (String, String) for error formatting;
// however, should be considered basically equivalent to a boolean.
impl<PD: PointData> From<&Glif<PD>> for Result<Forest<Component<PD>>, GlifParserError> {
    fn from(glif: &Glif<PD>) -> Self {
        let mut unique_found = None;

        fn component_to_tree<PD: PointData>(component: Component<PD>, glif: &Glif<PD>, uniques: &mut HashSet<String>, unique_found: &mut Option<(String, String)>) -> Result<Tree<Component<PD>>, GlifParserError> {
            let mut tree = Tree::new(component.clone());
            for gc in component.glif.components.iter() {
                let component_inner = gc.to_component_of(glif)?;
                if uniques.contains(&gc.base) {
                    return {
                        *unique_found = Some((component.glif.name.clone(), gc.base.clone()));
                        Ok(tree)
                    }
                }
                uniques.insert(gc.base.clone());
                tree.push_back(component_to_tree(component_inner, glif, uniques, unique_found)?);
            }
            Ok(tree)
        }

        let mut forest = Forest::new();
        let cs: Vec<_> = glif.components.iter().map(|gc| {
            let mut uniques = HashSet::new();
            uniques.insert(glif.name.clone());
            uniques.insert(gc.base.clone());
            component_to_tree(gc.to_component_of(glif).unwrap(), glif, &mut uniques, &mut unique_found).unwrap()
        }).collect();

        for c in cs {
            forest.push_back(c);
        }

        match unique_found {
            Some((base, unique)) => {Err(GlifParserError::GlifComponentsCyclical(format!("in glif {}, {} refers to {}", &glif.name, base, unique)))},
            None => Ok(forest)
        }
    }
}
