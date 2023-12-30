#![allow(dead_code)]

pub enum SymbolType {
    SymTypeUkn,
    SymTypeFunc   
}

pub struct Symbol {
    pub sym_tyep: SymbolType,
    pub name: String,
    pub address: u64
}