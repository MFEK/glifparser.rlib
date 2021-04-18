use glifparser;

#[test]
fn test_note() {
    let glifxml = r#"<?xml version="1.0" encoding="UTF-8"?>
<glyph name="note" format="2">
  <advance width="0" />
  <unicode hex="00" />
  <note>Hello world!</note>
  <outline />
  <!-- <MFEK></MFEK> -->
</glyph>"#;
    let glif: glifparser::Glif<()> = glifparser::glif::read(glifxml).unwrap();
    assert_eq!(glif.note, Some("Hello world!".to_string()));
    assert_eq!(glif.unicode[0], 0 as char);
    let newxml = glifparser::glif::write(&glif).unwrap();
    let newglif: glifparser::Glif<()> = glifparser::glif::read(&newxml).unwrap();
    assert_eq!(newglif.note, Some("Hello world!".to_string()));
    assert_eq!(newglif.unicode[0], 0 as char);
}
