use std::convert::TryInto;
use std::path;

use log::warn;

use super::Glif;
use crate::error::{GlifParserError::{self, GlifInputError}};
use crate::component::GlifComponent;
use crate::outline::{self, get_outline_type, GlifContour, GlifOutline, OutlineType};
use crate::point::{GlifPoint, PointData, parse_point_type};
use crate::anchor::Anchor;
use crate::image::GlifImage;

macro_rules! input_error {
    ($str:expr) => {
        GlifInputError($str.to_string())
    }
}

// Both components and images have the same matrix/identifier values. This is DRY.
macro_rules! load_matrix_and_identifier {
    ($xml_el:ident, $struct:ident) => {
        $xml_el.attributes.get("xScale").map(|e|{ $struct.xScale = e.as_str().try_into().unwrap(); });
        $xml_el.attributes.get("xyScale").map(|e|{ $struct.xyScale = e.as_str().try_into().unwrap(); });
        $xml_el.attributes.get("yxScale").map(|e|{ $struct.yxScale = e.as_str().try_into().unwrap(); });
        $xml_el.attributes.get("yScale").map(|e|{ $struct.yScale = e.as_str().try_into().unwrap(); });
        $xml_el.attributes.get("xOffset").map(|e|{ $struct.xOffset = e.as_str().try_into().unwrap(); });
        $xml_el.attributes.get("yOffset").map(|e|{ $struct.yOffset = e.as_str().try_into().unwrap(); });
        $xml_el.attributes.get("identifier").map(|e|{ $struct.identifier = Some(e.clone()); });
    }
}

// From .glif XML, return a parse tree
/// Read UFO .glif XML to Glif struct
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

    let mut images: Vec<GlifImage> = Vec::new();

    while let Some(image_el) = glif.take_child("image") {
        let filename = path::PathBuf::from(image_el
            .attributes
            .get("fileName")
            .ok_or(input_error!("<image> missing x"))?);

        let mut gimage = GlifImage::from_filename(filename)?;

        load_matrix_and_identifier!(image_el, gimage);

        images.push(gimage);
    }

    ret.images = images;

    let mut goutline: GlifOutline = Vec::new();

    let outline_el = glif.take_child("outline");

    if outline_el.is_some() {
        let mut outline_elu = outline_el.unwrap();
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
            load_matrix_and_identifier!(component_el, gcomponent);
            gcomponent.base = component_el.attributes.get("base").ok_or(input_error!("<component> missing base"))?.clone();
            ret.components.push(gcomponent);
        }
    }

    if let Some(lib) = glif.take_child("lib") {
        ret.lib = Some(lib);
    }

    // This will read the first XML comment understandable as itself containing XML.
    for child in &glif.children {
        if let xmltree::XMLNode::Comment(c) = child {
            let tree = xmltree::Element::parse(c.as_bytes());
            match tree {
                Ok(plib) => {
                    ret.private_lib = Some(plib);
                    break
                },
                Err(_) => {
                    warn!("Private dictionary found but unreadable");
                }
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
