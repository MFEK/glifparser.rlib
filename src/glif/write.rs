use xmltree;

use super::Glif;
use crate::error::GlifParserError;
use crate::point::{Handle, PointData, PointType};
use crate::codepoint::Codepoint;

use std::fs;
use std::path::Path;

fn build_ufo_point_from_handle(handle: Handle) -> Option<xmltree::Element>
{
    match handle {
        Handle::At(x, y) => {
            let mut glyph = xmltree::Element::new("point");
            glyph.attributes.insert("x".to_owned(), x.to_string());
            glyph.attributes.insert("y".to_owned(), y.to_string());
            return Some(glyph);
        },
        _ => {}
    }

    None
}

// Both components and images have the same matrix/identifier values. This is DRY.
macro_rules! write_matrix_and_identifier {
    ($xml_el:ident, $struct:ident) => {
        match $struct.identifier {
            Some(ref id) => {$xml_el.attributes.insert("identifier".to_string(), id.clone());},
            None  => {}
        }
        $xml_el.attributes.insert("xScale".to_string(), $struct.xScale.to_string());
        $xml_el.attributes.insert("xyScale".to_string(), $struct.xyScale.to_string());
        $xml_el.attributes.insert("yxScale".to_string(), $struct.yxScale.to_string());
        $xml_el.attributes.insert("yScale".to_string(), $struct.yScale.to_string());
        $xml_el.attributes.insert("xOffset".to_string(), $struct.xOffset.to_string());
        $xml_el.attributes.insert("yOffset".to_string(), $struct.yOffset.to_string());
    }
}

pub fn write_ufo_glif_to_filename<F, PD>(glif: &Glif<PD>, filename: F) -> Result<(), GlifParserError>
where F: AsRef<Path> + Clone, PD: PointData
{
    let glifxml: String = write_ufo_glif(glif)?;
    fs::write(filename, glifxml).or( Err(GlifParserError::XmlWriteError( "Failed to write to filename".to_string() )) )
}

pub fn write_ufo_glif<PD: PointData>(glif: &Glif<PD>) -> Result<String, GlifParserError>
{
    let ret = write_ufo_glif_data(glif)?;
    Ok(String::from_utf8(ret)?)
}

/// Write Glif struct to UFO .glif XML 
pub fn write_ufo_glif_data<PD: PointData>(glif: &Glif<PD>) -> Result<Vec<u8>, GlifParserError>
{
    let mut glyph = xmltree::Element::new("glyph");
        glyph.attributes.insert("name".to_owned(), glif.name.to_string());
        glyph.attributes.insert("format".to_owned(), "2".to_string());

    match glif.width {
        Some(w) => {
            let mut advanceel = xmltree::Element::new("advance");
            advanceel.attributes.insert("width".to_owned(), w.to_string());
            glyph.children.push(xmltree::XMLNode::Element(advanceel));
        },
        None => {}
    };

    for hex in glif.unicode.iter() {
        let mut unicode = xmltree::Element::new("unicode");
        unicode.attributes.insert("hex".to_owned(), (hex as &dyn Codepoint).display());
        glyph.children.push(xmltree::XMLNode::Element(unicode));
    }

    for anchor in glif.anchors.iter() {
        let mut anchor_node = xmltree::Element::new("anchor");
        anchor_node.attributes.insert("x".to_owned(), anchor.x.to_string());
        anchor_node.attributes.insert("y".to_owned(), anchor.y.to_string());
        anchor_node.attributes.insert("name".to_owned(), anchor.class.to_string());
        glyph.children.push(xmltree::XMLNode::Element(anchor_node));
    }

    let mut outline_node = xmltree::Element::new("outline");
    match &glif.outline
    {
        Some(outline) => {
            for contour in outline {
                // if we find a move point at the start of things we set this to false
                let open_contour = contour.first().unwrap().ptype == PointType::Move;
                let mut contour_node = xmltree::Element::new("contour");
                
                let mut last_point = None;
                // a is next, b is prev
                for (i, point) in contour.iter().enumerate() {
                    let mut point = point.clone();
                    if let Some(_lp) = last_point {
                        // if there was a point prior to this one we emit our b handle
                        if let Some(handle_node) = build_ufo_point_from_handle(point.b) {
                            contour_node.children.push(xmltree::XMLNode::Element(handle_node));
                        }
                    }
                    
                    // If the last point has a handle, the first point should be made a Curve (in
                    // case it already isn't). (fixup)
                    if i == 0 && !open_contour {
                        contour.last().map(|p| {
                            if p.a != Handle::Colocated {
                                point.ptype = PointType::Curve;
                            } else {
                                point.ptype = PointType::Line;
                            }
                        });
                    }

                    let mut point_node = xmltree::Element::new("point");
                    debug_assert!(point.is_valid(None));
                    point_node.attributes.insert("x".to_owned(), point.x.to_string());
                    point_node.attributes.insert("y".to_owned(), point.y.to_string());

                    let ptype_string = point.ptype.to_string();
                    match ptype_string.as_ref() {
                        "offcurve" => {} // while this name is OK, most often not written
                        _ => {point_node.attributes.insert("type".to_owned(), ptype_string);},
                    }
            
                    match &point.name {
                        Some(name) => {point_node.attributes.insert("name".to_owned(), name.to_string());},
                        None => {}
                    }

                    if point.smooth {
                        point_node.attributes.insert("smooth".to_owned(), "yes".to_owned());
                    }
                
                    // Point<T> does not contain field for identifier.
                    contour_node.children.push(xmltree::XMLNode::Element(point_node));
                    match point.ptype {
                        PointType::Line | PointType::Curve | PointType::Move => {
                            if let Some(handle_node) = build_ufo_point_from_handle(point.a) {
                                contour_node.children.push(xmltree::XMLNode::Element(handle_node));
                            }
                        },
                        PointType::QCurve => {
                            unimplemented!("Quadratic curves not writable yet")
                        },
                        _ => { return Err(GlifParserError::GlifOutlineHasBadPointType{idx: i, ptype: point.ptype}); }
                    }    
                    
                    last_point = Some(point);
                }

                // if a move wasn't our first point then we gotta close the shape by emitting the first point's b (prev) handle
                if !open_contour {
                    if let Some(handle_node) = build_ufo_point_from_handle(contour.first().unwrap().b) {
                        contour_node.children.push(xmltree::XMLNode::Element(handle_node));
                    }
                }

                outline_node.children.push(xmltree::XMLNode::Element(contour_node));
            }

        },
        None => {}
    }

    for component in &glif.components.vec {
        let mut component_node = xmltree::Element::new("component");
        component_node.attributes.insert("base".to_string(), component.base.clone());
        write_matrix_and_identifier!(component_node, component);
        outline_node.children.push(xmltree::XMLNode::Element(component_node));
    }

    glyph.children.push(xmltree::XMLNode::Element(outline_node));

    #[cfg(feature = "glifimage")]
    for image in &glif.images {
        let mut image_node = xmltree::Element::new("image");
        image_node.attributes.insert("fileName".to_string(), image.filename.to_str().ok_or(GlifParserError::GlifFilenameInsane("image filename not UTF8!".to_string()))?.to_string());
        write_matrix_and_identifier!(image_node, image);
        glyph.children.push(xmltree::XMLNode::Element(image_node));
    }

    for guideline in &glif.guidelines {
        let mut guideline_node = xmltree::Element::new("guideline");
        guideline_node.attributes.insert("x".to_string(), guideline.at.x.to_string());
        guideline_node.attributes.insert("y".to_string(), guideline.at.y.to_string());
        guideline_node.attributes.insert("angle".to_string(), guideline.angle.to_string());
        if let Some(c) = guideline.color {
            guideline_node.attributes.insert("color".to_string(), c.to_string());
        }
        if let Some(n) = &guideline.name {
            guideline_node.attributes.insert("name".to_string(), n.clone());
        }
        if let Some(i) = &guideline.identifier {
            guideline_node.attributes.insert("identifier".to_string(), i.clone());
        }
        glyph.children.push(xmltree::XMLNode::Element(guideline_node));
    }

    if let Some(note) = &glif.note {
        let mut note_node = xmltree::Element::new("note");
        note_node.children.push(xmltree::XMLNode::Text(note.clone()));
        glyph.children.push(xmltree::XMLNode::Element(note_node));
    }

    match &glif.lib {
        #[cfg(feature = "glifserde")]
        Some(lib_node) => {
            let mut plist_buf: Vec<u8> = vec![];
            plist::to_writer_xml(&mut plist_buf, lib_node)?;
            let lib = xmltree::Element::parse(plist_buf.as_slice())?;
            glyph.children.push(xmltree::XMLNode::Element(lib));
        }
        #[cfg(not(feature = "glifserde"))]
        Some(_) => {
            panic!("Tried to get XML from plist lib when not compiled with glifserde feature! Crashing to prevent data loss.")
        }
        None => {}
    }

    let config = xmltree::EmitterConfig::new()
        .perform_indent(true)
        .pad_self_closing(false)
        .autopad_comments(false);

    let mut ret_string: Vec<u8> = Vec::new();
    glyph.write_with_config(&mut ret_string, config)?;

    Ok(ret_string)
}
