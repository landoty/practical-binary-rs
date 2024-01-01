mod binary;

use std::fs;

// exposed fn to load a binary file and return a ptr to representative Binary struct on the heap
pub fn load_binary<'a>(fname: &'a str) -> Box<binary::Binary> {
    let file = fs::read(fname).unwrap();
    let object = object::File::parse(&*file);

    let mut binary: Box<binary::Binary> = Box::new(
        binary::Binary{
            filename: String::from(fname),
             ..Default::default()
            }
    );

    // binary module provides an unpacking fn that updates the struct using the Object crate
    match object {
        Ok(_) => binary::unpack_object(&mut binary, &object.unwrap()),
        Err(_) => panic!("Unable to parse {}", fname)
    };
    binary
}
