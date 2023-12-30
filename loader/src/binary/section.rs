#![allow(dead_code)]

pub enum SectionType {
    SecTypeNone,
    SecTypeCode,
    SecTypeData
}

pub struct Section {
    pub name: String,
    pub section_type: SectionType,
    pub vma: u64,
    pub size: u64,
    pub bytes: Vec<u8>
}

impl Section {
    pub fn contains(self, address: u64) -> bool {
        (address >= self.vma) && (address-self.vma < self.size)
    }
}