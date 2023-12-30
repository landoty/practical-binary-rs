mod binary;

use object::{Object, ObjectSection};
use std::fs;

pub fn test_load_binary(path: &str) {
    let binary = fs::read(path).unwrap();
    let object = object::File::parse(&*binary);
    for section in object.unwrap().sections() {
        println!("{}", section.name().unwrap());
    }
}

