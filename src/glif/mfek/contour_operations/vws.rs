use serde::{Serialize, Deserialize};
use std::{fmt::Display, str::FromStr};
use crate::{error::GlifParserError, glif::{MFEKContour, contour::MFEKContourCommon}};
use crate::glif::PointData;
use super::{ContourOperation, ContourOperations};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VWSContour {
    pub handles: Vec<VWSHandle>,
    pub join_type: JoinType,
    pub cap_start_type: CapType,
    pub cap_end_type: CapType,
    pub remove_internal: bool,
    pub remove_external: bool,
}


impl<PD: PointData> ContourOperation<PD> for VWSContour {
    fn sub(&mut self, begin: usize, end: usize) {
        let temp_handles = self.handles.split_at(begin);
        let (final_handles, _) = temp_handles.1.split_at(end + 1 - begin);

        self.handles = final_handles.into();
    }

    fn append(&mut self, append: &MFEKContour<PD>) {
        let mut temp_handles = self.handles.clone();

        match append.operation.clone() {
            Some(ContourOperations::VariableWidthStroke { mut data }) => {
                temp_handles.append(&mut data.handles)
            }
            Some(_) => {
                for _idx in 0..append.inner.len() {
                    let last_handle = *(temp_handles.last().unwrap_or(&VWSHandle {
                        left_offset: 10.,
                        right_offset: 10.,
                        tangent_offset: 0.,
                        interpolation: InterpolationType::Linear,
                    }));
                    temp_handles.push(last_handle);
                }
            }
            None => {
                for _idx in 0..append.inner.len() {
                    let last_handle = *(temp_handles.last().unwrap_or(&VWSHandle {
                        left_offset: 10.,
                        right_offset: 10.,
                        tangent_offset: 0.,
                        interpolation: InterpolationType::Linear,
                    }));
                    temp_handles.push(last_handle);
                }
            }
        }

        self.handles = temp_handles;
    }

    fn insert_op(&mut self, point_idx: usize) {
        self.handles.insert(
            point_idx,
            VWSHandle {
                left_offset: self.handles[point_idx].left_offset,
                right_offset: self.handles[point_idx].right_offset,
                tangent_offset: self.handles[point_idx].tangent_offset,
                interpolation: self.handles[point_idx].interpolation,
            },
        );
    }

    fn remove_op(&mut self, point_idx: usize) {
        self.handles.remove(point_idx);
    }
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct VWSHandle {
    pub left_offset: f64,
    pub right_offset: f64,
    pub tangent_offset: f64,
    pub interpolation: InterpolationType,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum JoinType {
    Bevel,
    Miter,
    Circle,
    Round,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            JoinType::Bevel => "bevel",
            JoinType::Miter => "miter",
            JoinType::Circle => "circle",
            JoinType::Round => "round",
        })
    }
}

impl FromStr for JoinType {
    type Err = GlifParserError/*::TypeConversionError(type, s)*/;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bevel" => Ok(JoinType::Bevel),
            "miter" => Ok(JoinType::Miter),
            "circle" => Ok(JoinType::Circle),
            "round" => Ok(JoinType::Round),
            _ => Err(GlifParserError::TypeConversionError{req_type: "JoinType", req_variant: s.to_owned()}),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CapType {
    Custom,
    Square,
    Circle,
    Round,
}

impl Display for CapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            CapType::Custom => "custom",
            CapType::Square => "square",
            CapType::Circle => "circle",
            CapType::Round => "round",
        })
    }
}

impl FromStr for CapType {
    type Err = GlifParserError/*::TypeConversionError(type, s)*/;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "custom" => Ok(CapType::Custom),
            "square" => Ok(CapType::Square),
            "circle" => Ok(CapType::Circle),
            "round" => Ok(CapType::Round),
            _ => {
                if s.ends_with(".glif") {
                    Ok(CapType::Custom)
                } else {
                    Err(GlifParserError::TypeConversionError{req_type: "CapType", req_variant: s.to_owned()})
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum InterpolationType {
    Null,
    Linear,
}

impl Display for InterpolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            InterpolationType::Null => "none",
            InterpolationType::Linear => "linear",
        })
    }
}

impl FromStr for InterpolationType {
    type Err = GlifParserError/*::TypeConversionError(type, s)*/;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(InterpolationType::Null),
            "linear" => Ok(InterpolationType::Linear),
            _ => Err(GlifParserError::TypeConversionError{req_type: "InterpolationType", req_variant: s.to_owned()}),
        }
    }
}