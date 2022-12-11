use crate::point::PointType;
use crate::point::Handle;
use crate::PointData;
use crate::WhichHandle;
use serde::{Serialize, Deserialize};

use super::MFEKPointCommon;

#[cfg_attr(feature = "glifserde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct QPoint<PD: PointData> {
    pub x: f32,
    pub y: f32,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub a: Handle,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub name: Option<String>,
    pub ptype: PointType,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub smooth: bool,
    #[cfg_attr(feature = "glifserde", serde(default))]
    pub data: Option<PD>,
}

impl<PD: PointData> QPoint<PD> {
    pub fn new() -> QPoint<PD> {
        Self::default()
    }

    /// Make a point from its x and y position and type
    pub fn from_x_y_type((x, y): (f32, f32), ptype: PointType) -> QPoint<PD> {
        QPoint { x, y, ptype, ..Default::default() }
    }
}

impl<PD: PointData> MFEKPointCommon<PD> for QPoint<PD> {
    fn get_handle(&self, wh: WhichHandle) -> Option<Handle> {
        if let WhichHandle::A = wh {
            return Some(self.a)
        } else {
            return None
        }
    }

    fn set_handle(&mut self, wh: WhichHandle, handle: Handle) {
        if let WhichHandle::A = wh {
            self.a = handle
        }
    }

    fn get_handle_position(&self, wh: WhichHandle) -> Option<(f32, f32)> {
        if let Some(Handle::At(x, y)) = self.get_handle(wh) {
            return Some((x, y))
        }

        None
    }

    fn set_handle_position(&mut self, wh: WhichHandle, x: f32, y: f32) {
        if let WhichHandle::A = wh {
            self.a = Handle::At(x, y);
        }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn set_position(&mut self, x: f32, y:f32) {
        let (cx, cy) = (self.x, self.y);
        let (dx, dy) = (cx - x, cy - y);
    
        self.x = x;
        self.y = y;
    
        let a = self.a;

        if let Handle::At(hx, hy) = a {
            self.a = Handle::At(hx - dx, hy - dy)
        }
    }

    fn set_position_no_handles(&mut self, x: f32, y:f32) {
        self.x = x;
        self.y = y;
    }

    fn cubic(&self) -> Option<&crate::Point<PD>> {
        None
    }

    fn quad(&self) -> Option<&QPoint<PD>> {
        Some(self)
    }
}