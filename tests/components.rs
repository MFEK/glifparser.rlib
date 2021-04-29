use glifparser;
use glifparser::FlattenedGlif;
use trees::{Tree, walk::{TreeWalk, Visit}};

fn pprint_component_tree(tree: Tree<glifparser::Component<()>>) {
    let mut tw: TreeWalk<glifparser::Component<()>> = tree.into();    
    let mut degree = 0;
    while let Some(visit) = tw.get() {
        match visit {
            Visit::Begin(node) => {eprintln!("{}{}{}", str::repeat(" ", degree), if degree > 0 { "⌞" } else { "" }, &node.data().glif.name); degree += 1;},
            Visit::Leaf(node) => {eprintln!("{}{}{}", str::repeat(" ", degree), if degree > 0 { "⌞" } else { "" }, &node.data().glif.name);},
            Visit::End(_) => {degree -= 1;}
        }
        tw.forward();
    }
}

#[test]
fn test_components() {
    let gliffn = "test_data/TT2020Base.ufo/glyphs/gershayim.glif";
    //let glifxml = fs::read_to_string(gliffn).unwrap();
    //let mut glif: glifparser::Glif<()> = glifparser::glif::read(&glifxml).unwrap();
    let mut glif: glifparser::Glif<()> = glifparser::glif::read_from_filename(gliffn).unwrap();
    glif.filename = Some(gliffn.into());
    let sanity = glif.filename_is_sane();
    assert!(sanity.is_ok() && sanity.unwrap());
    assert!(glif.components.vec.len() == 2);
    assert!(&glif.components.vec[0].base == "acute");
    let forest: Result<trees::Forest<glifparser::Component<()>>, _> = (&glif.components).into();
    match forest {
        Ok(mut f) => {
            eprintln!("(Glif: {})", &glif.name);
            while let Some(tree) = f.pop_front() {
                pprint_component_tree(tree);
            }
        },
        Err(e) => eprintln!("{}", e)
    }
    let flattened = glif.flattened(&mut None).unwrap();
    let flatxml = glifparser::glif::write(&flattened).unwrap();
    assert!(flatxml.len() > 0);
    //fs::write("/tmp/out.glif", flatxml);
}
