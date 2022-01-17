use std::path;

use crate::anchor::Anchor;
use crate::component::GlifComponents;
use crate::error::GlifParserError;
use crate::guideline::Guideline;
#[cfg(feature = "glifimage")]
use crate::image::GlifImage;
use crate::point::PointData;
use crate::outline::{Outline, OutlineType};

mod conv;
mod lib;
pub use lib::Lib;
mod read;
pub use self::read::read_ufo_glif as read;
pub use self::read::read_ufo_glif_pedantic as read_pedantic;
pub use self::read::read_ufo_glif_from_filename as read_from_filename;
pub use self::read::read_ufo_glif_from_filename_pedantic as read_from_filename_pedantic;
mod write;
pub use self::write::write_ufo_glif as write;
pub use self::write::write_ufo_glif_to_filename as write_to_filename;
#[cfg(feature = "mfek")]
pub mod mfek;
pub mod xml;
pub use self::{read::FromXML, write::IntoXML, xml::XMLConversion};

#[cfg(feature = "glifserde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct, ser::Error as SerdeError, de::Error as SerdeDeError};

#[cfg(feature = "mfek")]
pub use mfek::*;

/// A UFO .glif
///
/// TODO: use different generic types on Anchor and Guideline, making this declaration
/// `Glif<PD,GD,AD>`
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Glif<PD: PointData> {
    pub outline: Option<Outline<PD>>,
    pub order: OutlineType,
    pub anchors: Vec<Anchor<PD>>,
    /// Note that these components are not yet parsed or checked for infinite loops. You need to
    /// call either ``GlifComponent::to_component_of`` on each of these, or ``Glif::flatten``.
    pub components: GlifComponents,
    /// .glif guidelines. Note: glif may have more guidelines, not listed here. It will also have
    /// an asecender and a descender, not listed here. You can get this info from `norad`, reading
    /// the parent UFO and telling it not to read glif's (via UfoDataRequest) since you're using
    /// this for that.
    // Command line MFEK programs can also get it from MFEKmetadata.
    pub guidelines: Vec<Guideline<PD>>,
    /// glifparser does support reading the data of images and guessing their format, but in order
    /// to allow you to handle possibly erroneous files we don't do so by default. You need to call
    /// ``GlifImage::to_image_of`` to get an ``Image`` with data.
    #[cfg(feature = "glifimage")]
    pub images: Vec<GlifImage>,
    pub width: Option<u64>,
    pub unicode: Vec<char>,
    pub name: String,
    /// This is an arbitrary glyph comment, exactly like the comment field in FontForge SFD.
    pub note: Option<String>,
    /// It's up to the API consumer to set this.
    pub filename: Option<path::PathBuf>,
    /// glif private library
    pub lib: Lib,
}

#[cfg(feature = "glifserde")]
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

#[cfg(feature = "glifserde")]
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
        Self::default()
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

#[inline]
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
