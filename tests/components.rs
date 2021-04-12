use std::fs;
use glifparser;
//use trees;

#[test]
fn test_components() {
    let gliffn = "test_data/TT2020Base.ufo/glyphs/gershayim.glif";
    let glifxml = fs::read_to_string(gliffn).unwrap();
    let mut glif: glifparser::Glif<()> = glifparser::glif::read(&glifxml).unwrap();
    glif.filename = Some(gliffn.into());
    let sanity = glif.filename_is_sane();
    assert!(sanity.is_ok() && sanity.unwrap());
    assert!(glif.components.len() == 2);
    //assert!(&glif.components[0].base == "acute");
    /*let forest: Result<trees::Forest<glifparser::Component<()>>, _> = (&glif).into();
    match forest {
        Ok(f) => eprintln!("{:?}", f),
        Err(e) => eprintln!("{}", e)
    }*/
    let flattened = glif.flatten().unwrap();
    let flatxml = glifparser::glif::write(&flattened).unwrap();
    assert!(flatxml.len() > 0);
    //fs::write("/tmp/out.glif", flatxml);
}
