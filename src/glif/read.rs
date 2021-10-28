use std::convert::TryInto;
use std::path;
use std::rc::Rc;

use integer_or_float::IntegerOrFloat;
#[cfg(feature = "glifserde")]
use serde_json;

use super::Glif;
use crate::error::GlifParserError::{self, GlifInputError};
use crate::component::GlifComponent;
use crate::guideline::Guideline;
use crate::outline::{self, get_outline_type, GlifContour, GlifOutline, OutlineType};
use crate::point::{GlifPoint, PointData, parse_point_type};
use crate::anchor::Anchor;
#[cfg(feature = "glifimage")]
use crate::image::GlifImage;

macro_rules! input_error {
    ($str:expr) => {
        GlifInputError($str.to_string())
    }
}

// Both components and images have the same matrix/identifier values. This is DRY.
macro_rules! load_matrix_and_identifier {
    ($xml_el:ident, $struct:ident, ($($attr:ident),+)) => {
        $(
            let maybe_err = $xml_el.attributes.get(stringify!($attr)).map(|e| -> Result<(),GlifParserError> { 
                let v = e.as_str().try_into().or(Err(input_error!(concat!("Matrix member ", stringify!($attr), " not float"))))?;
                $struct.$attr = v;
                Ok(())
            });
            if let Some(Err(e)) = maybe_err { Err(e)?; };
        )+
        $xml_el.attributes.get("identifier").map(|e|{ $struct.identifier = Some(e.clone()); });
    }
}

use std::fs;
/// If you have a known filename, it is always preferable to call this function, as it sets the
/// filename on the Glif<PD> as well as on its GlifComponent's, easing their transition into
/// Component's.
pub fn read_ufo_glif_from_filename<F: AsRef<path::Path> + Clone, PD: PointData>(filename: F) -> Result<Glif<PD>, GlifParserError> {
    let glifxml = match fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(ioe) => Err(GlifParserError::GlifFileIoError(Some(Rc::new(ioe))))?
    };
    let mut glif: Glif<PD> = read_ufo_glif(&glifxml)?;
    let filenamepb = filename.as_ref().to_path_buf();
    for component in glif.components.vec.iter_mut() {
        component.set_file_name(&filenamepb);
    }
    glif.filename = Some(filenamepb);
    Ok(glif)
}

/// Read UFO .glif XML to Glif struct. This should only be used when you have no known filename,
/// and the glif is unattached from a UFO.
pub fn read_ufo_glif<PD: PointData>(glif: &str) -> Result<Glif<PD>, GlifParserError> {
    let mut glif = xmltree::Element::parse(glif.as_bytes())?;

    let mut ret = Glif::new();

    if glif.name != "glyph" {
        return Err(input_error!("Root element not <glyph>"))
    }

    if glif.attributes.get("format").ok_or(input_error!("no format in <glyph>"))? != "2" {
        return Err(input_error!("<glyph> format not 2"))
    }

    ret.name = glif
        .attributes
        .get("name")
        .ok_or(input_error!("<glyph> has no name"))?
        .clone();
    ret.components.root = ret.name.clone();

    let advance = glif
        .take_child("advance");

    ret.width = if let Some(a) = advance {
        Some(a.attributes
        .get("width")
        .ok_or(input_error!("<advance> has no width"))?
        .parse()
        .or(Err(input_error!("<advance> width not int")))?)
    } else {
        None
    };

    let mut unicodes = vec![];
    while let Some(u) = glif.take_child("unicode") {
        let unicodehex = u
            .attributes
            .get("hex")
            .ok_or(input_error!("<unicode> has no hex"))?;
        unicodes.push(
            char::from_u32(
                u32::from_str_radix(unicodehex, 16)
                .or(Err(input_error!("<unicode> hex not int")))?
            )
            .ok_or(input_error!("<unicode> char conversion failed"))?,
        );
    }

    ret.unicode = unicodes;

    let mut anchors: Vec<Anchor> = Vec::new();

    while let Some(anchor_el) = glif.take_child("anchor") {
        let mut anchor = Anchor::new();

        anchor.x = anchor_el
            .attributes
            .get("x")
            .ok_or(input_error!("<anchor> missing x"))?
            .parse()
            .or(Err(input_error!("<anchor> x not float")))?;
        anchor.y = anchor_el
            .attributes
            .get("y")
            .ok_or(input_error!("<anchor> missing y"))?
            .parse()
            .or(Err(input_error!("<anchor> y not float")))?;
        anchor.class = anchor_el
            .attributes
            .get("name")
            .ok_or(input_error!("<anchor> missing class"))?
            .clone();
        anchors.push(anchor);
    }

    ret.anchors = anchors;

    #[cfg(feature = "glifimage")] {
    let mut images: Vec<GlifImage> = Vec::new();

    while let Some(image_el) = glif.take_child("image") {
        let filename = path::PathBuf::from(image_el
            .attributes
            .get("fileName")
            .ok_or(input_error!("<image> missing fileName"))?);

        let mut gimage = GlifImage::from_filename(filename)?;

        load_matrix_and_identifier!(image_el, gimage, (xScale, xyScale, yxScale, yScale, xOffset, yOffset));

        if let Some(color) = image_el.attributes.get("color") {
            gimage.color = Some(color.parse()?);
        }

        images.push(gimage);
    }

    ret.images = images;
    }

    let mut guidelines: Vec<Guideline> = Vec::new();

    while let Some(guideline_el) = glif.take_child("guideline") {
        let gx = guideline_el
            .attributes
            .get("x")
            .ok_or(input_error!("<guideline> missing x"))?
            .parse()
            .or(Err(input_error!("<guideline> x not float")))?;
        let gy = guideline_el
            .attributes
            .get("y")
            .ok_or(input_error!("<guideline> missing y"))?
            .parse()
            .or(Err(input_error!("<guideline> x not float")))?;
        let angle: IntegerOrFloat = guideline_el
            .attributes
            .get("angle")
            .ok_or(input_error!("<guideline> missing angle"))?
            .as_str()
            .try_into()
            .or(Err(input_error!("<guideline> angle not float")))?;

        let mut guideline = Guideline::from_x_y_angle(gx, gy, angle);

        if let Some(color) = guideline_el.attributes.get("color") {
            guideline.color = Some(color.parse()?);
        }

        guideline.name = guideline_el.attributes.get("name").map(|n|n.clone());

        guideline.identifier = guideline_el.attributes.get("identifier").map(|i|i.clone());

        guidelines.push(guideline);
    }

    ret.guidelines = guidelines;

    if let Some(note_el) = glif.take_child("note") {
        note_el.get_text().map(|t|ret.note=Some(t.into_owned()));
    }

    let mut goutline: GlifOutline = Vec::new();

    let outline_el = glif.take_child("outline");

    if let Some(mut outline_elu) = outline_el {
        while let Some(mut contour_el) = outline_elu.take_child("contour") {
            let mut gcontour: GlifContour = Vec::new();
            while let Some(point_el) = contour_el.take_child("point") {
                let mut gpoint = GlifPoint::new();

                gpoint.x = point_el
                    .attributes
                    .get("x")
                    .ok_or(input_error!("<point> missing x"))?
                    .parse()
                    .or(Err(input_error!("<point> x not float")))?;
                gpoint.y = point_el
                    .attributes
                    .get("y")
                    .ok_or(input_error!("<point> missing y"))?
                    .parse()
                    .or(Err(input_error!("<point> y not float")))?;

                match point_el.attributes.get("name") {
                    Some(p) => gpoint.name = Some(p.clone()),
                    None => {}
                }

                gpoint.ptype =
                    parse_point_type(point_el.attributes.get("type").as_ref().map(|s| s.as_str()));

                gcontour.push(gpoint);
            }
            if gcontour.len() > 0 {
                goutline.push(gcontour);
            }
        }
        
        while let Some(component_el) = outline_elu.take_child("component") {
            let mut gcomponent = GlifComponent::new();
            load_matrix_and_identifier!(component_el, gcomponent, (xScale, xyScale, yxScale, yScale, xOffset, yOffset));
            gcomponent.base = component_el.attributes.get("base").ok_or(input_error!("<component> missing base"))?.clone();
            ret.components.vec.push(gcomponent);
        }
    }

    if let Some(lib) = glif.take_child("lib") {
        ret.lib = Some(lib);
    }

    // This will read the first XML comment understandable as containing JSON.
    #[cfg(feature = "glifserde")]
    for child in &glif.children {
        if let xmltree::XMLNode::Comment(c) = child {
            // This just checks if it's deserializable at all. serde_json::Value can contain any
            // JSON structure. The deserialization for real will happen in either mfek.rs or your
            // own hook into this, and the type being deserialized to will be changed to match.
            if !c.trim().starts_with(ret.private_lib_root) { log::warn!("Skipped comment not marked {}", ret.private_lib_root); continue }
            let comment_slice = &(c.trim()[ret.private_lib_root.len()..]);
            let de: Result<serde_json::Value, _> = serde_json::from_str(comment_slice);
            match de {
                Ok(_) => ret.private_lib = Some(comment_slice.to_string()),
                Err(e) => log::warn!("Skipped comment in input glif ({}) that was invalid JSON due to {:?}", &c, e)
            }
        }
    }

    ret.order = get_outline_type(&goutline);

    let outline = match ret.order {
        OutlineType::Cubic => outline::create::cubic_outline(&goutline),
        OutlineType::Quadratic => outline::create::quadratic_outline(&goutline),
        OutlineType::Spiro => Err(input_error!("Spiro as yet unimplemented"))?,
    };

    if outline.len() > 0 {
        ret.outline = Some(outline);
    }

    Ok(ret)
}
