use super::*;
use kurbo::Point as KurboPoint;

pub trait IsValid: Debug {
    /// You'll need to redefine this if you want to use it on PointData.
    fn is_valid(&self) -> bool { true }
    fn expect_valid(&self) {
        if !self.is_valid() {
            panic!("Failed validity check! {:?}", self)
        }
    }
}

pub trait PointLike: IsValid {
    fn x(&self) -> IntegerOrFloat;
    fn y(&self) -> IntegerOrFloat;
    fn set_x(&mut self, x: IntegerOrFloat);
    fn set_y(&mut self, y: IntegerOrFloat);
    fn x32(&self) -> f32 { f32::from(self.x()) }
    fn y32(&self) -> f32 { f32::from(self.y()) }
    fn x64(&self) -> f64 { f64::from(self.x()) }
    fn y64(&self) -> f64 { f64::from(self.y()) }
    fn as_kpoint(&self) -> KurboPoint {
        KurboPoint::new(self.x64(), self.y64())
    }
}

impl<PD: PointData> IsValid for Point<PD> {
    /// `validate_data` parameter allows you to define an `is_valid` (or whatever) impl on your
    /// `PointData` struct's. You can then pass the function while validating the point as e.g.
    /// `Some(MyPointData::is_valid)`. It takes an `Option<&PD>` so that you have the choice as to
    /// whether it's valid or not for your type not to be defined; `Point.data` should probably not
    /// be defined as an Option<PD>, but removing that's TODO. This API will change when that does
    /// and should be considered unstable/testing.
    fn is_valid(&self) -> bool {
        for pd in self.data.values() {
            if !pd.is_valid() {
                return false;
            }
        }

        if self.ptype == PointType::Undefined {
            return false;
        }
        if self.x.is_nan() || self.y.is_subnormal() {
            return false;
        }
        for handle in [self.handle(WhichHandle::A), self.handle(WhichHandle::B)] {
            if let Handle::At(hx, hy) = handle {
                if hx.is_nan() || hy.is_subnormal() {
                    return false;
                }
            }
        }
        true
    }
}

impl<PD: PointData> PointLike for Point<PD> {
    fn x(&self) -> IntegerOrFloat {
        IntegerOrFloat::from(self.x)
    }
    fn y(&self) -> IntegerOrFloat {
        IntegerOrFloat::from(self.y)
    }
    fn set_x(&mut self, x: IntegerOrFloat) {
        self.x = f32::from(x);
    }
    fn set_y(&mut self, y: IntegerOrFloat) {
        self.y = f32::from(y);
    }
}

impl IsValid for GlifPoint {
    fn is_valid(&self) -> bool {
        if self.ptype == PointType::Undefined {
            return false;
        }
        if self
            .x
            .holding_float()
            .map(|x| x.is_subnormal() || x.is_nan())
            .unwrap_or(false)
        {
            return false;
        }
        if self
            .y
            .holding_float()
            .map(|y| y.is_subnormal() || y.is_nan())
            .unwrap_or(false)
        {
            return false;
        }
        true
    }
}

impl PointLike for GlifPoint {
    fn x(&self) -> IntegerOrFloat {
        self.x
    }
    fn y(&self) -> IntegerOrFloat {
        self.y
    }
    fn set_x(&mut self, x: IntegerOrFloat) {
        self.x = x;
    }
    fn set_y(&mut self, y: IntegerOrFloat) {
        self.y = y;
    }
}
