use serde::Serialize;
use serde::Deserialize;
use kurbo::Affine;
use plist;

use super::{MFEKOutline, MFEKContour, LayerOperation};
use crate::color::Color;
use crate::glif::{Glif, name_to_filename};
use crate::image::GlifImage;
use crate::point::PointData;

macro_rules! DEFAULT_LAYER_FORMAT_STR {() => {"Layer {}"}}
pub const DEFAULT_LAYER_FORMAT_STR: &str = DEFAULT_LAYER_FORMAT_STR!();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer<PD: PointData> {
    pub name: String,
    pub visible: bool,
    pub color: Option<Color>,
    pub outline: MFEKOutline<PD>,
    pub operation: Option<LayerOperation>,
    pub images: Vec<(GlifImage, Affine)>,
}

impl<PD: PointData> Layer<PD> {
    pub fn to_glyphs_dir(&self, idx: usize) -> String {
        if idx == 0 {
            String::from("glyphs")
        } else {
            format!("glyphs.{}", name_to_filename(&self.name, false))
        }
    }
}

/// Create UFO(3) specification layer layerinfo.plist's for our glyph.
pub trait ToLayerInfoPlist {
    type Output = plist::Value;
    /// # Safety
    ///
    /// plist::Value guaranteed to be of variant plist::Dictionary
    fn to_layerinfo_plist(&self) -> Option<Self::Output>;
}

/// Create UFO(3) specification layer layercontents.plist's for our glyph.
pub trait ToLayerContentsPlist {
    type Output = plist::Value;
    /// # Safety
    ///
    /// plist::Value guaranteed to be of variant plist::Array
    fn to_layercontents_plist(&self) -> Self::Output;

    /// # Safety
    ///
    /// plist::Value guaranteed to be of variant plist::Array
    fn merge_layercontents_plists(&self, other: Self::Output) -> Self::Output;
}

impl<PD: PointData> ToLayerInfoPlist for Layer<PD> {
    fn to_layerinfo_plist(&self) -> Option<Self::Output> {
        let color = if let Some(color) = self.color {
            color
        } else {
            return None
        };

        let mut layerinfo: plist::Dictionary = plist::Dictionary::new();

        layerinfo.insert(String::from("color"), plist::Value::String(color.to_string()));

        Some(plist::Value::Dictionary(layerinfo))
    }
}

impl<PD: PointData> ToLayerContentsPlist for &[Layer<PD>] {
    fn to_layercontents_plist(&self) -> Self::Output {
        let mut ret: Vec<plist::Value> = Vec::new();

        for (i, layer) in self.iter().enumerate() {
            if !layer.visible { continue }
            let key = if layer.name == format!(DEFAULT_LAYER_FORMAT_STR!(), 0) {
                String::from("public.default") // https://unifiedfontobject.org/versions/ufo3/layercontents.plist/#publicdefault
            } else {
                layer.name.clone()
            };
            let value = layer.to_glyphs_dir(i);
            ret.push(plist::Value::Array(vec![plist::Value::String(key), plist::Value::String(value)]));
        }

        plist::Value::Array(ret)
    }
    fn merge_layercontents_plists(&self, other: Self::Output) -> Self::Output {
        let our_lc = self.to_layercontents_plist();
        let mut inner_ours: Vec<plist::Value> = our_lc.into_array().unwrap(); // Safe. Cf. to_layercontents_plist return type
        let mut inner_theirs: Vec<plist::Value> = other.into_array().unwrap();
        for l in inner_ours.iter() {
            if !inner_theirs.contains(&l) {
                inner_theirs.push(l.clone());
            }
        }
        plist::Value::Array(inner_theirs)
    }
}
