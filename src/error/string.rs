#[derive(Debug, Display, Error)]
pub enum GlifStringConversionError {
    #[display(fmt="String has control character `{:?}` at position {}", c, idx)]
    HasControlCharacter {
        idx: usize,
        c: char
    },
    #[display(fmt="String is zero-length")]
    LenZero,
}
