use std::fs;
use std::path;
use integer_or_float::IntegerOrFloat;
use glifparser;
use glifparser::{Image, Color, image::DataOrBitmap};
use log;

use test_log::test;

#[test]
fn test_load_image() {
    let gliffn = "test_data/TT2020Base.ufo/glyphs/N_U_L_L_.glif";
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

#[test]
fn test_image_png() {
    let mut image = Image::from_filename("test_data/logo.png").unwrap();
    image.color = Some( Color{ r: IntegerOrFloat::Integer(0), g: IntegerOrFloat::Integer(0), b: IntegerOrFloat::Integer(1), a: IntegerOrFloat::Integer(1) } );
    image.decode().unwrap();
    match image.data.data {
        //DataOrBitmap::Bitmap { pixels, width, height } => eprintln!("{:?} {}x{}", pixels, width, height),
        DataOrBitmap::Bitmap { width, height, .. } => log::info!("{}x{}", width, height),
        _ => panic!("Decode failed")
    }
}
