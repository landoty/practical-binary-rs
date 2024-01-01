mod binary;

use std::fs;

// there really should be a heap allocation in here somewhere

fn load_binary_object<'a>(fname: &str, bin_type: binary::BinaryType) -> Box<binary::Binary> {
    let file = fs::read(fname).unwrap();
    let object = object::File::parse(&*file);

    let mut binary: Box<binary::Binary> = Box::new(
        binary::Binary{
            filename: String::from(fname),
            bin_type,
             ..Default::default()
            }
    );


    match object {
        Ok(_) => binary::unpack_object(&mut binary, &object.unwrap()),
        Err(_) => panic!("Unable to parse {}", fname)
    };
    return binary;
}

pub fn load_binary<'a>(fname: &'a str) -> Box<binary::Binary> {
    let bin: Box<binary::Binary> = load_binary_object(fname, binary::BinaryType::BinTypeAuto);
    println!("Binary: {}\t Entry: {}", bin.filename, bin.entry);
    bin
}
