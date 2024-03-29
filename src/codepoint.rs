pub trait Codepoint {
    fn display(&self) -> String;
}

impl Codepoint for char {
    fn display(&self) -> String {
        format!("{:x}", *self as u32)
    }
}
