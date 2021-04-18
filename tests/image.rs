use std::fs;
use std::path;
use glifparser;

use env_logger;

fn init() {
    let _ = env_logger::builder().init();
}

#[test]
fn test_image() {
    init();

    let gliffn = "test_data/TT2020Base.ufo/glyphs/eight.glif";
    let glif_outxml;
    // Read
    {
        let glifxml = fs::read_to_string(gliffn).unwrap();
        let mut glif: glifparser::Glif<()> = glifparser::glif::read(&glifxml).unwrap();
        glif.filename = Some(path::PathBuf::from(gliffn));
        let im = glif.images[0].to_image_of(&glif).unwrap();
        assert_eq!(im.codec, glifparser::ImageCodec::WebP);
        assert_eq!(im.data().unwrap().len(), 44370);
        // Write
        glif_outxml = glifparser::write(&glif).unwrap();
    }
    // Test read back
    let mut glif_roundtrip: glifparser::Glif<()> = glifparser::read(&glif_outxml).unwrap();
    glif_roundtrip.filename = Some(path::PathBuf::from(gliffn));
    let im = glif_roundtrip.images[0].to_image_of(&glif_roundtrip).unwrap();
    assert_eq!(im.codec, glifparser::ImageCodec::WebP);
    assert_eq!(im.data().unwrap().len(), 44370);
}
