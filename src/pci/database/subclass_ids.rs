use enum_iterator::IntoEnumIterator;

use super::class_ids::PCI_CLASS_ID;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
pub enum PCI_SUBCLASS_UNCLASSIFIED {
    VGACompatibleUnclassifiedDevice = 0x00,
    NonVGACompatibleUnclassifiedDevice = 0x01,
}
