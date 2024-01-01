extern crate loader;

use loader::{load_binary};

fn main() {
    let bin = load_binary("/home/landen/pba-rs/loader/src/test/plain");
    println!("{}", bin);
}
