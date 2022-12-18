use crate::point::Handle;
use crate::PointData;
use crate::WhichHandle;
use serde::{Serialize, Deserialize};

use super::MFEKPointCommon;
use super::quad::QPoint;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct HyperPoint<PD> {
    pub x: f32,
    pub y: f32,

    pub a: Handle,
    pub b: Handle,

    pub kind: HyperPointType,

    pub smooth: bool,

    pub data: Option<PD>,
}

impl<PD: PointData> HyperPoint<PD> {
    pub fn new(x: f32, y: f32, kind: HyperPointType, smooth: bool) -> Self{
        Self {
            x,
            y,
            kind,
            smooth,
            ..Default::default()
        }
    }
}

impl<PD: PointData> MFEKPointCommon<PD> for HyperPoint<PD> {
    fn get_handle(&self, wh: WhichHandle) -> Option<Handle> {
        match wh {
            WhichHandle::Neither => None,
            WhichHandle::A => Some(self.a),
            WhichHandle::B => Some(self.b),
        }
    }

    fn set_handle(&mut self, wh: WhichHandle, handle: Handle) {
        match wh {
            WhichHandle::Neither => {},
            WhichHandle::A => {self.a = handle}
            WhichHandle::B => {self.b = handle}
        }
    }

    fn get_handle_position(&self, wh: WhichHandle) -> Option<(f32, f32)> {
        if let Some(Handle::At(x, y)) = self.get_handle(wh) {
            return Some((x, y))
        }

        None
    }

    fn set_handle_position(&mut self, wh: WhichHandle, x: f32, y: f32) {
        self.set_handle(wh, Handle::At(x, y));
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn get_name(&self) -> Option<String> {
        None
    }

    fn set_name(&mut self, _name: String) {
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
        let b = self.b;
        match a {
            Handle::At(hx, hy) => self.a = Handle::At(hx - dx, hy - dy),
            Handle::Colocated => (),
        }
        match b {
            Handle::At(hx, hy) => self.b = Handle::At(hx - dx, hy - dy),
            Handle::Colocated => (),
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
        None
    }

    fn hyper(&self) -> Option<&HyperPoint<PD>> {
        return Some(self)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum HyperPointType {
    #[default]
    Curve,
    Line
}