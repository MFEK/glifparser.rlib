use glifparser;

#[super::test]
fn test_bug_fixed() {
    let gliffn = "test_data/bugfixes/issue54.glif";
    log::trace!("{}", gliffn);
    let glif: glifparser::Glif<()> = glifparser::glif::read_from_filename(gliffn).unwrap();
    assert_eq!(glif.width, Some(1143));
    let gliffn = "test_data/bugfixes/Q_.glif";
    log::trace!("{}", gliffn);
    let glif: Result<glifparser::Glif<()>, glifparser::error::GlifParserError> = glifparser::glif::read_from_filename(gliffn);
    assert!(glif.is_err());
}
