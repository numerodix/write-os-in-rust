use alloc::{format, string::String};
use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
#[repr(u8)]
pub enum PCI_CLASS_ID {
    Unclassified = 0x00,
    MassStorageController = 0x01,
    NetworkController = 0x02,
    DisplayController = 0x03,
    MultimediaController = 0x04,
    MemoryController = 0x05,
    Bridge = 0x06,
    SimpleCommunicationController = 0x07,
    BaseSystemPeripheral = 0x08,
    InputDeviceController = 0x09,
    DockingStation = 0x0A,
    Processor = 0x0B,
    SerialBusController = 0x0C,
    WirelessController = 0x0D,
    IntelligentController = 0x0E,
    SatelliteCommunicationController = 0x0F,
    EncryptionController = 0x10,
    SignalProcessingController = 0x11,
    ProcessingAccelerator = 0x12,
    NonEssentialInstrumentation = 0x13,
    // reserved 0x14 - 0x3F
    CoProcessor = 0x40,
    // reserved 0x41 - 0xFE
    Unassigned = 0xFF,
}

pub fn get_class_name(class_id: u8) -> Option<String> {
    for id in PCI_CLASS_ID::into_enum_iter() {
        if class_id == id as u8 {
            return Some(format!("{:?}", id));
        }
    }

    None
}
