use alloc::{format, string::String};
use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
#[repr(u16)]
pub enum PCI_SUBCLASS_ID {
    // Unclassified
    VGACompatibleUnclassifiedDevice = 0x0000,
    NonVGACompatibleUnclassifiedDevice = 0x0001,

    // MassStorageController
    SCSIBusController = 0x0100,
    IDEController = 0x0101,
    FloppyDiskController = 0x0102,
    IPIBusController = 0x0103,
    RADIController = 0x0104,
    ATAController = 0x0105,
    SerialATAController = 0x0106,
    SerialAttachedSCSIController = 0x0107,
    NonVolatileMemoryController = 0x0108,
    // - Other = 0x0180,

    // NetworkController
    EthernetController = 0x0200,
    TokenRingController = 0x0201,
    FDDIController = 0x0202,
    ATMController = 0x0203,
    ISDNController = 0x0204,
    WorldFipController = 0x0205,
    PICMGController = 0x0206,
    InfinibandController = 0x0207,
    FabricController = 0x0208,
    // - Other = 0x0280,

    // DisplayController
    VGACompatibleController = 0x0300,
    XGAController = 0x0301,
    ThreeDController = 0x0302,
    // - Other = 0x0380,
}

pub fn get_subclass_name(class_id: u8, subclass_id: u8) -> Option<String> {
    for id in PCI_SUBCLASS_ID::into_enum_iter() {
        let key: u16 = (class_id as u16) << 8 | (subclass_id as u16);
        if key == id as u16 {
            return Some(format!("{:?}", id));
        }
    }

    None
}
