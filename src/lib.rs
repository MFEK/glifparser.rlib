#![feature(let_chains)]
extern crate log;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointType {
    Undefined,
    Move,
    Curve,
    QCurve,
    QClose,
    Line,
    OffCurve,
} // Undefined used by new(), shouldn't appear in Point<PointData> structs

#[derive(Debug, Copy, Clone)]
pub enum AnchorType {
    Undefined,
    Mark,
    Base,
    MarkMark,
    MarkBase,
} // Undefined used everywhere for now as getting type requires parsing OpenType features, which we will be using nom to do since I have experience w/it.

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Handle {
    Colocated,
    At(f32, f32),
}

impl From<Option<&GlifPoint>> for Handle {
    fn from(point: Option<&GlifPoint>) -> Handle {
        match point {
            Some(p) => Handle::At(p.x, p.y),
            None => Handle::Colocated,
        }
    }
}

// A "close to the source <point>"
#[derive(Clone, Debug)]
struct GlifPoint {
    x: f32,
    y: f32,
    smooth: bool,
    name: Option<String>,
    ptype: PointType,
}

impl GlifPoint {
    fn new() -> GlifPoint {
        GlifPoint {
            x: 0.,
            y: 0.,
            ptype: PointType::Undefined,
            smooth: false,
            name: None,
        }
    }
}

type GlifContour = Vec<GlifPoint>;
type GlifOutline = Vec<GlifContour>;
pub type PointData = dyn Clone;

// A Skia-friendly point
#[derive(Debug, Clone)]
pub struct Point<PointData> {
    pub x: f32,
    pub y: f32,
    pub a: Handle,
    pub b: Handle,
    pub name: Option<String>,
    pub ptype: PointType,
    pub data: Option<PointData>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WhichHandle {
    Neither,
    A,
    B,
}

impl<PointData> Point<PointData> {
    pub fn new() -> Point<PointData> {
        Point {
            x: 0.,
            y: 0.,
            a: Handle::Colocated,
            b: Handle::Colocated,
            ptype: PointType::Undefined,
            name: None,
            data: None,
        }
    }

    pub fn from_x_y_type(at: (f32, f32), ptype: PointType) -> Point<PointData> {
        Point {
            x: at.0,
            y: at.1,
            a: Handle::Colocated,
            b: Handle::Colocated,
            ptype: ptype,
            name: None,
            data: None,
        }
    }

    pub fn handle_or_colocated(
        &self,
        which: WhichHandle,
        transform_x: fn(f32) -> f32,
        transform_y: fn(f32) -> f32,
    ) -> (f32, f32) {
        let handle = match which {
            WhichHandle::A => self.a,
            WhichHandle::B => self.b,
            WhichHandle::Neither => Handle::Colocated,
        };
        match handle {
            Handle::At(x, y) => (transform_x(x), transform_y(y)),
            Handle::Colocated => (transform_x(self.x), transform_y(self.y)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Anchor {
    pub x: f32,
    pub y: f32,
    pub class: String,
    pub r#type: AnchorType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IntegerOrFloat {
    Integer(i32),
    Float(f32)
}

#[allow(non_snake_case)] // to match UFO spec https://unifiedfontobject.org/versions/ufo3/glyphs/glif/#component
#[derive(Clone, Debug)]
pub struct GlifComponent {
    base: String,
    xScale: IntegerOrFloat,
    xyScale: IntegerOrFloat,
    yScale: IntegerOrFloat,
    xOffset: IntegerOrFloat,
    yOffset: IntegerOrFloat,
    identifier: Option<String>
}

impl Anchor {
    pub fn new() -> Anchor {
        Anchor {
            x: 0.,
            y: 0.,
            r#type: AnchorType::Undefined,
            class: String::new(),
        }
    }
}

pub type Contour<PointData> = Vec<Point<PointData>>;
pub type Outline<PointData> = Vec<Contour<PointData>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutlineType {
    Cubic,
    Quadratic,
    // As yet unimplemented.
    // Will be in <lib> with cubic Bezier equivalents in <outline>.
    Spiro,
}

pub trait Codepoint {
    fn display(&self) -> String;
}

impl Codepoint for char {
    fn display(&self) -> String {
        format!("{:x}", *self as u32)
    }
}

#[derive(Clone, Debug)]
pub struct Glif<PointData> {
    pub outline: Option<Outline<PointData>>,
    pub order: OutlineType,
    pub anchors: Vec<Anchor>,
    pub width: Option<u64>,
    pub unicode: Vec<char>,
    pub name: String,
    pub format: u8, // we only understand 2
    pub filename: Option<String>,
    pub lib: Option<xmltree::Element>
}


extern crate xmltree;
use std::collections::VecDeque;

#[allow(unused)]
fn parse_anchor(anchor_el: xmltree::Element) -> Result<Anchor, &'static str> {
    todo!()
}

fn parse_point_type(pt: Option<&str>) -> PointType {
    match pt {
        Some("move") => PointType::Move,
        Some("line") => PointType::Line,
        Some("qcurve") => PointType::QCurve,
        Some("curve") => PointType::Curve,
        _ => PointType::OffCurve,
    }
}

fn midpoint(x1: f32, x2: f32, y1: f32, y2: f32) -> (f32, f32) {
    ((x1 + x2) / 2., (y1 + y2) / 2.)
}

fn get_outline_type(goutline: &GlifOutline) -> OutlineType {
    for gc in goutline.iter() {
        for gp in gc.iter() {
            match gp.ptype {
                PointType::Curve => return OutlineType::Cubic,
                PointType::QCurve => return OutlineType::Quadratic,
                _ => {}
            }
        }
    }

    OutlineType::Cubic // path has no off-curve point, only lines
}

// UFO uses the same compact format as TTF, so we need to expand it.
fn create_quadratic_outline<PointData>(goutline: &GlifOutline) -> Outline<PointData> {
    let mut outline: Outline<PointData> = Vec::new();

    let mut temp_outline: VecDeque<VecDeque<GlifPoint>> = VecDeque::new();

    let mut stack: VecDeque<&GlifPoint> = VecDeque::new();

    for gc in goutline.iter() {
        let mut temp_contour = VecDeque::new();

        for gp in gc.iter() {
            match gp.ptype {
                PointType::OffCurve => {
                    stack.push_back(&gp);
                }
                _ => {}
            }

            if stack.len() == 2 {
                let h1 = stack.pop_front().unwrap();
                let h2 = stack.pop_front().unwrap();
                let mp = midpoint(h1.x, h2.x, h1.y, h2.y);

                temp_contour.push_back(h1.clone());
                temp_contour.push_back(GlifPoint {
                    x: mp.0,
                    y: mp.1,
                    ptype: PointType::QCurve,
                    smooth: true,
                    name: gp.name.clone(),
                });
                stack.push_back(h2);
            } else if gp.ptype != PointType::OffCurve {
                let h1 = stack.pop_front();
                match h1 {
                    Some(h) => temp_contour.push_back(h.clone()),
                    _ => {}
                }
                temp_contour.push_back(gp.clone());
            }
        }
        if let (Some(h1), Some(h2)) = (stack.pop_front(), temp_contour.get(0)) {
            let mp = midpoint(h1.x, h2.x, h1.y, h2.y);
            let (t, tx, ty) = (h2.ptype, h2.x, h2.y);
            temp_contour.push_back(h1.clone());
            if t == PointType::OffCurve {
                temp_contour.push_back(GlifPoint {
                    x: mp.0,
                    y: mp.1,
                    ptype: PointType::QCurve,
                    smooth: true,
                    name: None,
                });
            } else {
                temp_contour.push_back(GlifPoint {
                    x: tx,
                    y: ty,
                    ptype: PointType::QClose,
                    smooth: true,
                    name: None,
                });
            }
        }

        temp_outline.push_back(temp_contour);
        assert_eq!(stack.len(), 0);
    }

    for gc in temp_outline.iter() {
        let mut contour: Contour<PointData> = Vec::new();

        for gp in gc.iter() {
            match gp.ptype {
                PointType::OffCurve => {
                    stack.push_back(&gp);
                }
                _ => {
                    assert!(stack.len() < 2);
                    let h1 = stack.pop_front();

                    if let Some(_) = h1 {
                        contour.last_mut().map(|p| p.a = Handle::from(h1));
                    }

                    contour.push(Point {
                        x: gp.x,
                        y: gp.y,
                        a: Handle::Colocated,
                        b: Handle::Colocated,
                        name: gp.name.clone(),
                        ptype: gp.ptype,
                        data: None,
                    });
                }
            }
        }

        if contour.len() > 0 {
            outline.push(contour);
        } else {
            log::warn!("Dropped empty contour. Lone `move` point in .glif? GlifContour: {:?}", &gc);
        }
    }

    outline
}

// Stack based outline builder. Push all offcurve points onto the stack, pop them when we see an on
// curve point. For each point, we add one handle to the current point, and one to the last. We
// then connect the last point to the first to make the loop, (if path is closed).
fn create_cubic_outline<PointData>(goutline: &GlifOutline) -> Outline<PointData> {
    let mut outline: Outline<PointData> = Vec::new();

    let mut stack: VecDeque<&GlifPoint> = VecDeque::new();

    for gc in goutline.iter() {
        let mut contour: Contour<PointData> = Vec::new();

        for gp in gc.iter() {
            match gp.ptype {
                PointType::OffCurve => {
                    stack.push_back(&gp);
                }
                PointType::Line | PointType::Move | PointType::Curve => {
                    let h1 = stack.pop_front();
                    let h2 = stack.pop_front();

                    contour.last_mut().map(|p| p.a = Handle::from(h1));

                    contour.push(Point {
                        x: gp.x,
                        y: gp.y,
                        a: Handle::Colocated,
                        b: Handle::from(h2),
                        name: gp.name.clone(),
                        ptype: gp.ptype,
                        data: None,
                    });
                }
                PointType::QCurve => {
                    unreachable!("Quadratic point in cubic glyph! File is corrupt.")
                }
                _ => {}
            }
        }

        let h1 = stack.pop_front();
        let h2 = stack.pop_front();

        contour.last_mut().map(|p| p.a = Handle::from(h1));

        if contour.len() > 0 && contour[0].ptype != PointType::Move {
            contour.first_mut().map(|p| p.b = Handle::from(h2));
        }

        if contour.len() == 1 && contour.first().unwrap().ptype == PointType::Move {
            log::warn!("Dropped empty contour. Lone `move` point in .glif? GlifContour: {:?}", &gc);
        }
        else if contour.len() > 0 {
            outline.push(contour);
        }
    }

    outline
}

// From .glif XML, return a parse tree
pub fn read_ufo_glif<PointData>(glif: &str) -> Glif<PointData> {
    let mut glif = xmltree::Element::parse(glif.as_bytes()).expect("Invalid XML");

    let mut ret = Glif {
        outline: None,
        order: OutlineType::Cubic, // default when only corners
        anchors: vec![],
        width: None,
        unicode: vec![],
        name: String::new(),
        format: 2,
        filename: None,
        lib: None
    };

    assert_eq!(glif.name, "glyph", "Root element not <glyph>");
    assert_eq!(
        glif.attributes
            .get("format")
            .expect("<glyph> has no format"),
        "2",
        "<glyph> format not 2"
    );

    ret.name = glif
        .attributes
        .get("name")
        .expect("<glyph> has no name")
        .clone();
    let advance = glif
        .take_child("advance");

    ret.width = advance.iter().next().map(|w| {
        w.attributes
        .get("width")
        .expect("<advance> has no width")
        .parse()
        .expect("<advance> width not int")});

    let mut unicodes = vec![];
    while let Some(u) = glif.take_child("unicode") {
        let unicodehex = u
            .attributes
            .get("hex")
            .expect("<unicode> has no hex");
        unicodes.push(
            char::from_u32(u32::from_str_radix(unicodehex, 16).expect("<unicode> hex not int"))
                .expect("<unicode> char conversion failed"),
        );
    }

    ret.unicode = unicodes;

    let mut anchors: Vec<Anchor> = Vec::new();

    while let Some(anchor_el) = glif.take_child("anchor") {
        let mut anchor = Anchor::new();

        anchor.x = anchor_el
            .attributes
            .get("x")
            .expect("<anchor> missing x")
            .parse()
            .expect("<anchor> x not float");
        anchor.y = anchor_el
            .attributes
            .get("y")
            .expect("<anchor> missing y")
            .parse()
            .expect("<anchor> y not float");
        anchor.class = anchor_el
            .attributes
            .get("name")
            .expect("<anchor> missing class")
            .clone();
        anchors.push(anchor);
    }

    ret.anchors = anchors;

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
                    .expect("<point> missing x")
                    .parse()
                    .expect("<point> x not float");
                gpoint.y = point_el
                    .attributes
                    .get("y")
                    .expect("<point> missing y")
                    .parse()
                    .expect("<point> y not float");

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
    }

    if let Some(lib) = glif.take_child("lib") {
        ret.lib = Some(lib);
    }

    ret.order = get_outline_type(&goutline);

    let outline = match ret.order {
        OutlineType::Cubic => create_cubic_outline(&goutline),
        OutlineType::Quadratic => create_quadratic_outline(&goutline),
        OutlineType::Spiro => unreachable!("Spiro as yet unimplemented"),
    };

    if outline.len() > 0 {
        ret.outline = Some(outline);
    }

    ret
}

fn point_type_to_string(ptype: PointType) -> Option<String>
{
    return match ptype{
        PointType::Undefined => None,
        PointType::OffCurve => None,
        PointType::QClose => None, // should probably be removed from PointType
        PointType::Move => Some(String::from("move")),
        PointType::Curve => Some(String::from("curve")),
        PointType::QCurve => Some(String::from("qcurve")),
        PointType::Line => Some(String::from("line")),
    }
}

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

pub fn write_ufo_glif<PointData>(glif: &Glif<PointData>) -> String
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

    match &glif.outline
    {
        Some(outline) => {
            let mut outline_node = xmltree::Element::new("outline");
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

            glyph.children.push(xmltree::XMLNode::Element(outline_node));
        },
        None => {}
    }

    match &glif.lib {
        Some(lib_node) => {
            glyph.children.push(xmltree::XMLNode::Element(lib_node.clone()));
        }
        None => {}
    }

    let mut ret_string: Vec<u8> = Vec::new();
    let config = xmltree::EmitterConfig::new().perform_indent(true);
    glyph.write_with_config(&mut ret_string, config).expect("Failed to write glyph!");

    return String::from_utf8(ret_string).unwrap();
}
