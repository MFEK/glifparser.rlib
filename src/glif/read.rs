mod xml;
pub use self::xml::FromXML;

use std::convert::TryInto;
use std::path;
use std::rc::Rc;

use integer_or_float::IntegerOrFloat;

use super::{Glif, Lib};
use crate::error::GlifParserError::{self, GlifInputError};
use crate::component::GlifComponent;
use crate::guideline::Guideline;
use crate::outline::{GlifContour, GlifOutline, Outline};
use crate::point::{GlifPoint, PointData, PointType};
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

    let mut anchors: Vec<Anchor<PD>> = Vec::new();

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

    let mut guidelines: Vec<Guideline<PD>> = Vec::new();

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

    let mut goutline: GlifOutline = GlifOutline::new();

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
                    Some(n) => gpoint.name = Some(n.clone()),
                    None => {}
                }

                gpoint.ptype = point_el.attributes.get("type").as_ref().map(|s| s.as_str()).unwrap_or("offcurve").into();

                match point_el.attributes.get("smooth") {
                    Some(s) => if s == "yes" {
                        if gpoint.ptype != PointType::OffCurve {
                            gpoint.smooth = true;
                        } else {
                            log::error!("Ignoring illogical `smooth=yes` on offcurve point");
                        }
                    },
                    _ => {}
                }

                if gpoint.ptype.is_valid() {
                    gcontour.push(gpoint);
                } else {
                    Err(GlifInputError(format!("Shouldn't write <point type={}> to UFO .glif!", gpoint.ptype)))?;
                }
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

    #[cfg(feature = "glifserde")]
    if let Some(lib) = glif.take_child("lib") {
        let mut plist_temp: Vec<u8> = vec![];
        match lib.write(&mut plist_temp).map(|()|plist::from_bytes(&plist_temp)) {
            Ok(Ok(lib_p)) => ret.lib = Lib::Plist(lib_p),
            Err(e) => {
                log::error!("Failed to serialize .glif lib as XML? Error: {:?}", e);
                ret.lib = Lib::Xml(lib)
            },
            Ok(Err(e)) => {
                log::error!("Failed to deserialize .glif lib XML as plist? Error: {:?}", e);
            }
        }
    }
    #[cfg(not(feature = "glifserde"))]
    if let Some(_) = glif.take_child("lib") {
        log::warn!("Without glifserde, cannot decode plist!")
    }

    goutline.figure_type();
    ret.order = goutline.otype.into();

    let outline: Outline<PD> = goutline.try_into()?;

    if outline.len() > 0 || ret.components.vec.len() > 0 {
        ret.outline = Some(outline);
    }

    Ok(ret)
}
