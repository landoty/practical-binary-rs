#![allow(dead_code)]

mod section;
mod symbol;

use object::{Architecture, Object, File};

pub enum BinaryType {
    BinTypeAuto,
    BinTypeNone,
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

impl BinaryType {
    pub fn to_str(&self) -> &'static str{
        match &self {
            BinaryType::BinTypeAuto => "auto",
            BinaryType::BinTypeElf => "elf",
            _ => "none"
        }
    }
}

impl BinaryArch {
    pub fn to_str(&self) -> &'static str{
        match &self {
            BinaryArch::ArchNone => "none",
            BinaryArch::ArchX86 => "x86",
        }
    }
}

impl Default for Binary {
    fn default() -> Self {
        Binary { 
            filename: String::from(""), 
            bin_type: BinaryType::BinTypeNone, 
            type_str: String::from(""), 
            arch: BinaryArch::ArchNone, 
            arch_str: String::from(""), 
            bits: 0, 
            entry: 0, 
            sections: Vec::new(), 
            symbols: Vec::new() }
    }
}

fn arch_from_object(arch: Architecture) -> BinaryArch {
    match arch {
        Architecture::X86_64 | Architecture::X86_64_X32 => BinaryArch::ArchX86,
        _ => BinaryArch::ArchNone
    }
}

pub fn unpack_object<'a>(bin:&'a mut Binary, obj: &File) {
    let arch = arch_from_object(obj.architecture());
    
    bin.type_str = String::from(bin.bin_type.to_str());
    bin.arch_str = String::from(arch.to_str());
    bin.arch = arch;
    bin.entry = obj.entry();
}