use super::gp;
use super::GPResult;

#[super::test]
fn test_bug_fixed() {
    let gliffn = "test_data/bugfixes/issue54.glif";
    log::trace!("{}", gliffn);
    let glif: gp::Glif<()> = gp::glif::read_from_filename(gliffn).unwrap();
    assert_eq!(glif.width, Some(1143));
    log::trace!("{}", gliffn);
    let glif: GPResult = gp::glif::read_from_filename_pedantic(gliffn, gp::Pedantry::new(gp::pedantry::Level::OpenType, gp::pedantry::Mend::Never));
    assert!(glif.is_err());
    log::trace!("{}", gliffn);
    let glif: gp::Glif<()> = gp::glif::read_from_filename_pedantic(gliffn, gp::Pedantry::default()).unwrap();
    assert_eq!(glif.width, Some(1143));
    log::trace!("{}", gliffn);
    let glif: GPResult = gp::glif::read_from_filename_pedantic(gliffn, gp::Pedantry::new(gp::pedantry::Level::OpenType, gp::pedantry::Mend::Never));
    assert!(glif.is_err());
    log::trace!("{}", gliffn);
    let glif: GPResult = gp::glif::read_from_filename_pedantic(gliffn, gp::Pedantry::default());
    let gliffn = "test_data/bugfixes/Q_.glif";
    log::trace!("{}", gliffn);
    let glif: GPResult = gp::glif::read_from_filename(gliffn);
    assert!(glif.is_err());
}
