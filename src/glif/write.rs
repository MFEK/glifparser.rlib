use xmltree;

use super::Glif;
use crate::error::GlifParserError;
use crate::point::{Handle, PointData, PointType, point_type_to_string};
use crate::codepoint::Codepoint;

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

/// Write Glif struct to UFO .glif XML 
pub fn write_ufo_glif<PD: PointData>(glif: &Glif<PD>) -> Result<String, GlifParserError>
{
    let mut glyph = xmltree::Element::new("glyph");
        glyph.attributes.insert("name".to_owned(), glif.name.to_string());
        glyph.attributes.insert("format".to_owned(), glif.format.to_string());

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
                for point in contour {
                    if let Some(_lp) = last_point {
                        // if there was a point prior to this one we emit our b handle
                        if let Some(handle_node) = build_ufo_point_from_handle(point.b) {
                            contour_node.children.push(xmltree::XMLNode::Element(handle_node));
                        }
                    }

                    let mut point_node = xmltree::Element::new("point");
                        point_node.attributes.insert("x".to_owned(), point.x.to_string());
                        point_node.attributes.insert("y".to_owned(), point.y.to_string());
                
                        match point_type_to_string(point.ptype) {
                            Some(ptype_string) => {point_node.attributes.insert("type".to_owned(), ptype_string);},
                            None => {}
                        }
                
                        match &point.name {
                            Some(name) => {point_node.attributes.insert("name".to_owned(), name.to_string());},
                            None => {}
                        }
                
                        // Point>T> does not contain fields for smooth, or identifier.
                    contour_node.children.push(xmltree::XMLNode::Element(point_node));
                    match point.ptype {
                        PointType::Line | PointType::Curve | PointType::Move => {
                            if let Some(handle_node) = build_ufo_point_from_handle(point.a) {
                                contour_node.children.push(xmltree::XMLNode::Element(handle_node));
                            }                        
                        },
                        PointType::QCurve => {
                            //QCurve currently unhandled. This needs to be implemented.
                        },
                        _ => { } // I don't think this should be reachable in a well formed Glif object?
                    }    
                    
                    last_point = Some(point);
                }

                // if a move wasn't our first point then we gotta close the shape by emitting the first point's b handle
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

    for component in &glif.components {
        let mut component_node = xmltree::Element::new("component");
        component_node.attributes.insert("base".to_string(), component.base.clone());
        match component.identifier {
            Some(ref id) => {component_node.attributes.insert("identifier".to_string(), id.clone());},
            None  => {}
        }
        component_node.attributes.insert("xScale".to_string(), component.xScale.to_string());
        component_node.attributes.insert("xyScale".to_string(), component.xyScale.to_string());
        component_node.attributes.insert("yxScale".to_string(), component.yxScale.to_string());
        component_node.attributes.insert("yScale".to_string(), component.yScale.to_string());
        component_node.attributes.insert("xOffset".to_string(), component.xOffset.to_string());
        component_node.attributes.insert("yOffset".to_string(), component.yOffset.to_string());
        outline_node.children.push(xmltree::XMLNode::Element(component_node));
    }

    glyph.children.push(xmltree::XMLNode::Element(outline_node));

    match &glif.lib {
        Some(lib_node) => {
            glyph.children.push(xmltree::XMLNode::Element(lib_node.clone()));
        }
        None => {}
    }

    let config = xmltree::EmitterConfig::new().perform_indent(false).write_document_declaration(false);

    match &glif.private_lib {
        Some(lib_node) => {
            let mut private_xml = Vec::new();
            lib_node.write_with_config(&mut private_xml, config)?;
            let private_xml = String::from_utf8(private_xml)?;
            glyph.children.push(xmltree::XMLNode::Comment(private_xml));
        },
        None => {}
    }

    let config = xmltree::EmitterConfig::new().perform_indent(true);

    let mut ret_string: Vec<u8> = Vec::new();
    glyph.write_with_config(&mut ret_string, config)?;

    return Ok(String::from_utf8(ret_string)?);
}
