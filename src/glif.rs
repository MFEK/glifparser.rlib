use std::path;
use xmltree;

use crate::anchor::Anchor;
use crate::component::GlifComponents;
use crate::error::GlifParserError;
use crate::guideline::Guideline;
use crate::image::GlifImage;
use crate::point::PointData;
use crate::outline::{Outline, OutlineType};

mod read;
mod write;
#[cfg(feature = "mfek")]
pub mod mfek;

pub use read::read_ufo_glif as read;
pub use read::read_ufo_glif_from_filename as read_from_filename;
pub use write::write_ufo_glif as write;
pub use write::write_ufo_glif_to_filename as write_to_filename;

use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use serde::ser::Error as SerdeError;
use serde::de::Error as SerdeDeError;

#[cfg(feature = "mfek")]
pub use mfek::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Glif<PD: PointData> {
    pub outline: Option<Outline<PD>>,
    pub order: OutlineType,
    pub anchors: Vec<Anchor>,
    /// Note that these components are not yet parsed or checked for infinite loops. You need to
    /// call either ``GlifComponent::to_component_of`` on each of these, or ``Glif::flatten``.
    pub components: GlifComponents,
    /// .glif guidelines. Note: glif may have more guidelines, not listed here. It will also have
    /// an asecender and a descender, not listed here. You can get this info from `norad`, reading
    /// the parent UFO and telling it not to read glif's (via UfoDataRequest) since you're using
    /// this for that.
    // Command line MFEK programs can also get it from MFEKmetadata.
    pub guidelines: Vec<Guideline>,
    /// glifparser does support reading the data of images and guessing their format, but in order
    /// to allow you to handle possibly erroneous files we don't do so by default. You need to call
    /// ``GlifImage::to_image_of`` to get an ``Image`` with data.
    pub images: Vec<GlifImage>,
    pub width: Option<u64>,
    pub unicode: Vec<char>,
    pub name: String,
    /// This is an arbitrary glyph comment, exactly like the comment field in FontForge SFD.
    pub note: Option<String>,
    pub format: u8, // we only understand 2
    /// It's up to the API consumer to set this.
    pub filename: Option<path::PathBuf>,
    /// We give you the <lib> as an XML Element. Note, however, that in the UFO spec it is a plist
    /// dictionary. You're going to need to parse this with a plist parser, such as plist.rs. You
    /// may want to tell xmltree to write it back to a string first; however, it may be possible to
    /// parse plist from xmltree::Element. Might change some day to a ``plist::Dictionary``.
    pub lib: Option<xmltree::Element>,
    /// This is a JSON structure that will be written into a comment in the .glif file.
    pub private_lib: Option<String>,
    /// This string helps prevent us from reading comments not intended for us into our
    /// private_lib.
    pub private_lib_root: &'static str,
}

impl<PD: PointData> Serialize for Glif<PD> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Glif", 1)?;
        let self_string = write(&self);
        if self_string.is_err() { return Err(SerdeError::custom("Could not serialize glif!")) }
        state.serialize_field("inner", &self_string.unwrap())?;
        state.end()
    }
}


impl<'de, PD: PointData> Deserialize<'de> for Glif<PD> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let loaded_glif = read(&s);
        if loaded_glif.is_err() { return Err(SerdeDeError::custom("Could not deserialize glif!")) }

        return Ok(loaded_glif.unwrap());
    }
}

impl<PD: PointData> Glif<PD> {
    pub fn new() -> Self {
        Glif {
            outline: None,
            order: OutlineType::Cubic, // default when only corners
            anchors: vec![],
            components: GlifComponents::new(),
            guidelines: vec![],
            images: vec![],
            width: None,
            unicode: vec![],
            name: String::new(),
            note: None,
            format: 2,
            filename: None,
            lib: None,
            private_lib: None,
            private_lib_root: "!!MFEK!!",
        }
    }

    pub fn name_to_filename(&self) -> String {
        name_to_filename(&self.name, true)
    }

    pub fn filename_is_sane(&self) -> Result<bool, GlifParserError> {
        match &self.filename {
            Some(gfn) => {
                let gfn_fn = match gfn.file_name() {
                    Some(gfn_fn) => gfn_fn,
                    None => { return Err(GlifParserError::GlifFilenameInsane("Glif file name is directory".to_string())) }
                };

                Ok(self.name_to_filename() == gfn_fn.to_str().ok_or(GlifParserError::GlifFilenameInsane("Glif file name has unknown encoding".to_string()))?)
            }
            None => Err(GlifParserError::GlifFilenameInsane("Glif file name is not set".to_string()))
        }
    }

}

impl<PD: PointData> Default for Glif<PD> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait GlifLike {
    fn name(&self) -> &String;
    fn filename(&self) -> &Option<path::PathBuf>;
}

impl<PD: PointData> GlifLike for Glif<PD> {
    fn name(&self) -> &String {
        &self.name
    }
    fn filename(&self) -> &Option<path::PathBuf> {
        &self.filename
    }
}

pub fn name_to_filename(name: &str, append_extension: bool) -> String {
    let mut ret = String::new();
    let chars: Vec<char> = name.chars().collect();
    for c in chars {
        ret.push(c);
        if ('A'..'Z').contains(&c) {
            ret.push('_');
        }
    }
    if append_extension {
        ret.push_str(".glif");
    }
    ret
}
