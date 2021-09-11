use alloc::{format, string::String};
use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
pub enum PCI_VENDOR_NAME {
    AMD,
    Broadcom,
    Dell,
    HP,
    Intel,
    IBM,
    Micron,
    Realtek,
    Ubiquitity,
    WesternDigital,
}

const VENDOR_MAP: [(u16, PCI_VENDOR_NAME); 12] = [
    (0x0777, PCI_VENDOR_NAME::Ubiquitity),
    (0x8086, PCI_VENDOR_NAME::Intel),
    (0x1000, PCI_VENDOR_NAME::Broadcom),
    (0x1002, PCI_VENDOR_NAME::AMD),
    (0x1014, PCI_VENDOR_NAME::IBM),
    (0x101C, PCI_VENDOR_NAME::WesternDigital),
    (0x1022, PCI_VENDOR_NAME::AMD),
    (0x1028, PCI_VENDOR_NAME::Dell),
    (0x103C, PCI_VENDOR_NAME::HP),
    (0x1042, PCI_VENDOR_NAME::Micron),
    (0x10EC, PCI_VENDOR_NAME::Realtek),
    (0xFEDA, PCI_VENDOR_NAME::Broadcom),
];

pub fn get_vendor_name(class_code: u16) -> Option<String> {
    for (code, id) in VENDOR_MAP.iter() {
        if class_code == *code {
            return Some(format!("{:?}", id));
        }
    }

    None
}
