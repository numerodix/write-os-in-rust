use alloc::{format, string::String};
use enum_iterator::IntoEnumIterator;

use super::class_ids::PCI_CLASS_ID;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
pub enum PCI_SUBCLASS_UNCLASSIFIED {
    VGACompatibleUnclassifiedDevice = 0x00,
    NonVGACompatibleUnclassifiedDevice = 0x01,
}

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
pub enum PCI_SUBCLASS_MASS_STORAGE_CONTROLLER {
    SCSIBusController = 0x00,
    IDEController = 0x01,
}

pub fn get_class_name(class_id: u8, subclass_id: u8) -> Option<String> {
    if class_id == PCI_CLASS_ID::Unclassified as u8 {
        for id in PCI_SUBCLASS_UNCLASSIFIED::into_enum_iter() {
            if subclass_id == id as u8 {
                return Some(format!("{:?}", id));
            }
        }
    }

    None
}
