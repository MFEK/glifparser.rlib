
use super::Glif;
#[derive(Debug, Clone)]
pub struct VWSContour {
    pub handles: Vec<VWSHandle>,
    pub join_type: JoinType,
    pub cap_start_type: CapType,
    pub cap_end_type: CapType
}

#[derive(Debug, Clone, Copy)]
pub enum InterpolationType {
    Linear,
    Null
}

#[derive(Debug, Clone, Copy)]
pub struct VWSHandle {
    pub left_offset: f64,
    pub right_offset: f64,
    pub tangent_offset: f64,
    pub interpolation: InterpolationType
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinType {
    Bevel,
    Miter,
    Round
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapType {
    Round,
    Square,
    Custom
}

pub fn parse_vws_op(vws: &mut xmltree::Element) -> VWSContour {
    let cap_start = vws
        .attributes
        .get("cap_start");

    let cap_end = vws
        .attributes
        .get("cap_end");

    let join = vws
        .attributes
        .get("join");

    let round_str ="round".to_string();
    let cap_start = cap_start.unwrap_or(&round_str);
    let cap_end = cap_end.unwrap_or(&round_str);
    let join = join.unwrap_or(&round_str);

    let cap_start_type = match cap_start.as_str() {
        "round" => CapType::Round,
        "square" => CapType::Square,
        "custom" => CapType::Custom,
        _ => panic!("Invalid cap type!")
    };

    let cap_end_type = match cap_end.as_str() {
        "round" => CapType::Round,
        "square" => CapType::Square,
        "custom" => CapType::Custom,
        _ => panic!("Invalid cap type!")
    };

    let join_type = match join.as_str() {
        "round" => JoinType::Round,
        "miter" => JoinType::Miter,
        "bevel" => JoinType::Bevel,
        _ => panic!("Invalid join type!")
    };

    let mut vws_handles = VWSContour {
        handles: Vec::new(),
        cap_start_type,
        cap_end_type,
        join_type
    };

    while let Some(vws_handle) = vws.take_child("handle") {
        let left: f64 = vws_handle
            .attributes
            .get("left")
            .expect("VWSHandle missing left")
            .parse()
            .expect("VWSHandle not float.");

        let right: f64 = vws_handle
            .attributes
            .get("right")
            .expect("VWSHandle missing right")
            .parse()
            .expect("VWSHandle not float.");

        let tangent: f64 = vws_handle
            .attributes
            .get("tangent")
            .expect("VWSHandle missing tangent")
            .parse()
            .expect("VWSHandle tangent not float.");

        let interpolation_string: &String = vws_handle
            .attributes
            .get("interpolation")
            .expect("VWSHandle missing interpolation type");
            

        let interpolation = match interpolation_string.as_str() {
            "linear" => InterpolationType::Linear,
            _ => InterpolationType::Null
        };

        vws_handles.handles.push(VWSHandle{
            left_offset: left,
            right_offset: right,
            tangent_offset: tangent,
            interpolation: interpolation
        });
    }

    return vws_handles;
}

pub fn write_vws_op(idx: usize, vwscontour: &VWSContour) -> xmltree::Element
{
    let mut vws_node = xmltree::Element::new("contour_op");
    vws_node.attributes.insert("type".to_owned(), "VWS".to_owned());
    vws_node.attributes.insert("contour_idx".to_owned(), idx.to_string());
    vws_node.attributes.insert("cap_start".to_owned(), cap_type_to_string(vwscontour.cap_start_type));
    vws_node.attributes.insert("cap_end".to_owned(), cap_type_to_string(vwscontour.cap_end_type));
    vws_node.attributes.insert("join".to_owned(), join_type_to_string(vwscontour.join_type));

   for handle in &vwscontour.handles {
       let mut handle_node = xmltree::Element::new("handle");
       handle_node.attributes.insert("left".to_owned(), handle.left_offset.to_string());
       handle_node.attributes.insert("right".to_owned(), handle.right_offset.to_string());
       handle_node.attributes.insert("tangent".to_owned(), handle.tangent_offset.to_string());


       match handle.interpolation {
           InterpolationType::Linear => {handle_node.attributes.insert("interpolation".to_owned(), "linear".to_owned());},
           InterpolationType::Null => {handle_node.attributes.insert("interpolation".to_owned(), "none".to_owned());}
       }
       
       vws_node.children.push(xmltree::XMLNode::Element(handle_node));
   }


   return vws_node;
}

pub fn cap_type_to_string(ct: CapType)  -> String
{
    match ct {
        CapType::Round => "round".to_string(),
        CapType::Square => "square".to_string(),
        CapType::Custom => "custom".to_string(),
    }
}

pub fn join_type_to_string(jt: JoinType)  -> String
{
    match jt {
        JoinType::Round => "round".to_string(),
        JoinType::Miter => "miter".to_string(),
        JoinType::Bevel => "bevel".to_string(),
    }
}