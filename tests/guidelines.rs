use integer_or_float::IntegerOrFloat;
use glifparser;

#[test]
fn test_note() {
    let glifxml = r#"<?xml version="1.0" encoding="UTF-8"?>
<glyph name="guidelines" format="2">
  <advance width="0" />
  <unicode hex="00" />
  <guideline x="5" y="9.2432" angle="180" name="gl" color="1,0,0,1" />
  <guideline x="50.0" y="9" angle="90.0" name="gl2" color="1,0,0,1" />
  <outline />
  <!-- <MFEK></MFEK> -->
</glyph>"#;
    let glif: glifparser::Glif<()> = glifparser::glif::read(glifxml).unwrap();
    assert_eq!(glif.guidelines.len(), 2);
    assert_eq!(glif.guidelines[0].name, Some(String::from("gl")));
    assert_eq!(glif.guidelines[0].at, glifparser::GuidelinePoint{x: 5., y: 9.2432});
    assert!(glif.guidelines[1].angle == IntegerOrFloat::Float(90.0) || glif.guidelines[1].angle == IntegerOrFloat::Integer(90));
    let newxml = glifparser::glif::write(&glif).unwrap();
    let newglif: glifparser::Glif<()> = glifparser::glif::read(&newxml).unwrap();
    assert_eq!(newglif.guidelines.len(), 2);
    assert_eq!(newglif.guidelines[0].name, Some(String::from("gl")));
    assert_eq!(newglif.guidelines[0].at, glifparser::GuidelinePoint{x: 5., y: 9.2432});
    assert!(newglif.guidelines[1].angle == IntegerOrFloat::Float(90.0) || newglif.guidelines[1].angle == IntegerOrFloat::Integer(90));
}
