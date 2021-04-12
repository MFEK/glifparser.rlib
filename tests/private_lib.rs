use xmltree;
use glifparser;

#[test]
fn test_roundtrip_mfek_private_lib() {
    let mut glif: glifparser::Glif<()> = glifparser::Glif::new();
    let mut root = xmltree::Element::new("MFEK");
    let mut el = xmltree::Element::new("test");
    el.attributes.insert("equals".to_string(), "3<>-<>".to_string());
    root.children.push(xmltree::XMLNode::Element(el));
    glif.private_lib = Some(root);
    let xml = glifparser::glif::write(&glif).unwrap();
    eprintln!("{}",&xml);
    let glif2: glifparser::Glif<()> = glifparser::glif::read(&xml).unwrap();
    let xml2 = glifparser::glif::write(&glif2).unwrap();
    assert_eq!(glif, glif2);
    assert_eq!(xmltree::Element::parse(xml.as_bytes()).unwrap(), xmltree::Element::parse(xml2.as_bytes()).unwrap());
}
