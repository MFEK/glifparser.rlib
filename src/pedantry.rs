//! When reading .glif files, how strict ought we to be? Can we make fixes to bad input, or ought
//! we to error out and make the user do it?
use integer_or_float::IntegerOrFloat;

#[derive(Constructor, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Pedantry {
    pub level: Level,
    pub mend: Mend,
}

impl Pedantry {
    pub fn should_mend(&self) -> bool {
        !(self.level.is_sfnt() && self.mend.is_never())
    }
}

#[derive(Derivative, Debug, Copy, Clone, PartialEq, Eq, IsVariant, Unwrap)]
#[derivative(Default)]
pub enum Mend {
    #[derivative(Default(new="true"))]
    Always,
    Never,
    UfoSpecErrorsOnly,
    UfoSpecOutdatedOnly,
}

#[derive(Derivative, Debug, Copy, Clone, PartialEq, Eq, IsVariant, Unwrap)]
#[derivative(Default)]
pub enum Level {
    #[derivative(Default(new="true"))]
    /// Glifparser's permissive attitude to the spec
    GlifParser,
    /// Strict to the UFO spec
    Ufo,
    /// Strict to a UFO that will be used to make an OpenType font. For example, this places limits
    /// on anchor values: a UfoPedantic `<anchor>` can have float placement, while an
    /// OpenTypePedantic anchor cannot.
    OpenType,
    /// Strict to a UFO that will be used to make a TrueType font. This will refuse floats
    /// everywhere but matrices, with the more important consequence being every point location
    /// gets rounded to integer.
    TrueType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, IsVariant, Unwrap)]
pub enum FloatClass {
    Anchor,
    AdvanceWidth
}

impl FloatClass {
    fn should_round(&self) -> bool {
        *self == FloatClass::Anchor
    }
}

impl Level {
    pub fn maybe_round(&self, f: IntegerOrFloat, fc: FloatClass) -> f32 {
        if self.is_sfnt() && fc.should_round() {
            f32::from(f).round()
        } else {
            f.into()
        }
    }

    pub fn is_sfnt(&self) -> bool {
        self.is_open_type() || self.is_true_type()
    }
}
