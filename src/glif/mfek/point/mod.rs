use crate::{WhichHandle, PointData, Handle, Point};

pub mod quad;
pub mod hyper;

use dyn_clone::DynClone;

use self::{quad::QPoint, hyper::HyperPoint};

pub trait MFEKPointCommon<PD: PointData>: DynClone {
    fn get_handle(&self, handle: WhichHandle) -> Option<Handle>;
    fn set_handle(&mut self, wh: WhichHandle, handle: Handle);
    fn get_handle_position(&self, handle: WhichHandle) -> Option<(f32, f32)>;
    fn set_handle_position(&mut self, handle: WhichHandle, x: f32, y: f32);
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn get_name(&self) -> Option<String>;
    fn set_name(&mut self, name: String);
    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, x: f32, y:f32);
    fn set_position_no_handles(&mut self, x: f32, y:f32);
    fn cubic(&self) -> Option<&Point<PD>>;
    fn quad(&self) -> Option<&QPoint<PD>>;
    fn hyper(&self) -> Option<&HyperPoint<PD>>;
}

impl<PD: PointData> MFEKPointCommon<PD> for Point<PD> {
    fn get_handle(&self, handle: WhichHandle) -> Option<Handle> {
        match handle {
            WhichHandle::Neither => None,
            WhichHandle::A => Some(self.a),
            WhichHandle::B => Some(self.b),
        }
    }

    fn get_handle_position(&self, handle: WhichHandle) -> Option<(f32, f32)> {
        let handle_option = self.get_handle(handle);
    
        if let Some(handle) = handle_option {
            match handle {
                Handle::At(hx, hy) => {
                    return Some((hx, hy));
                }
                Handle::Colocated => return None,
            }
        }

        None
    }

    fn set_handle_position(&mut self, handle: WhichHandle, x: f32, y: f32) {
        match handle {
            WhichHandle::Neither => {},
            WhichHandle::A => {
                self.a = Handle::At(x, y)
            },
            WhichHandle::B => {
                self.b = Handle::At(x, y)
            },
        }        
    }

    fn set_handle(&mut self, wh: WhichHandle, handle: Handle) {
        match wh {
            WhichHandle::Neither => {},
            WhichHandle::A => self.a = handle,
            WhichHandle::B => self.b = handle,
        }
    }

    fn get_position(&self) -> (f32, f32) {
        return (self.x, self.y)
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

    fn x(&self) -> f32 {
        return self.x;
    }

    fn y(&self) -> f32 {
        return self.y;
    }

    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    fn cubic(&self) -> Option<&Point<PD>> {
        return Some(self);
    }
    
    fn quad(&self) -> Option<&QPoint<PD>> {
        None
    }

    fn hyper(&self) -> Option<&HyperPoint<PD>> {
        None
    }
}
