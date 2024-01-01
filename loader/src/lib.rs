mod binary;

use std::fs;
use object::{Architecture, Object, File};

fn unpack_object<'a>(bin:&'a mut binary::Binary, obj: &File) {
    let arch = match obj.architecture() {
        Architecture::X86_64 | Architecture::X86_64_X32 => binary::BinaryArch::ArchX86,
        _ => binary::BinaryArch::ArchNone
    };
    
    bin.type_str = String::from(bin.bin_type.to_str());
    bin.arch_str = String::from(arch.to_str());
    bin.arch = arch;
    bin.entry = obj.entry();
    bin.bits = if obj.is_64() { 64 } else { 32 }; 
}

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
        Ok(_) => unpack_object(&mut binary, &object.unwrap()),
        Err(_) => panic!("Unable to parse {}", fname)
    };
    binary
}
