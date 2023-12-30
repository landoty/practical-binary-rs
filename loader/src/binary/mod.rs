#![allow(dead_code)]

mod section;
mod symbol;

pub enum BinaryType {
    BinTypeAuto,
    BinTypeElf
}

pub enum BinaryArch {
    ArchNone,
    ArchX86
}

pub struct Binary {
    pub filename: String,
    pub bin_type: BinaryType,
    pub type_str: String,
    pub arch: BinaryArch,
    pub arch_str: String,
    pub bits: u8,
    pub entry: u64,
    pub sections: Vec<section::Section>,
    pub symbols: Vec<symbol::Symbol>
}