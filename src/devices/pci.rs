use crate::serial_println;
use alloc::{format, string::String};
use enum_iterator::IntoEnumIterator;
use x86_64::instructions::port::Port;

pub const CONFIG_ADDRESS: u16 = 0xCF8;
pub const CONFIG_DATA: u16 = 0xCFC;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
#[repr(u16)]
pub enum PCI_VENDOR_ID {
    AdvancedMicroDevices = 0x1022,
    Intel = 0x8086,
    Realtek = 0x10EC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciVendor {
    Known(PCI_VENDOR_ID),
    Unknown(u16),
}

impl PciVendor {
    fn display(&self) -> String {
        match self {
            Self::Known(_) => format!("{:?}", self),
            Self::Unknown(id) => format!("Unknown(0x{:x})", id),
        }
    }
}

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
#[repr(u8)]
pub enum PCI_DEVICE_CLASS {
    Unclassified = 0x00,
    MassStorageController = 0x01,
    NetworkController = 0x02,
    DisplayController = 0x03,
    MultimediaController = 0x04,
    MemoryController = 0x05,
    Bridge = 0x06,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciDeviceClass {
    Known(PCI_DEVICE_CLASS),
    Unknown(u8),
}

impl PciDeviceClass {
    fn display(&self) -> String {
        match self {
            Self::Known(_) => format!("{:?}", self),
            Self::Unknown(id) => format!("Unknown(0x{:x})", id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDevice {
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
}

impl PciDevice {
    fn display(&self) -> String {
        let mut vendor_id = PciVendor::Unknown(self.vendor);
        let mut class_id = PciDeviceClass::Unknown(self.class);

        for id in PCI_VENDOR_ID::into_enum_iter() {
            if self.vendor == id as u16 {
                vendor_id = PciVendor::Known(id);
            }
        }

        for id in PCI_DEVICE_CLASS::into_enum_iter() {
            if self.class == id as u8 {
                class_id = PciDeviceClass::Known(id);
            }
        }

        format!(
            "PciDevice {{ vendor: {}, device: 0x{:x}, class: {} }}",
            vendor_id.display(),
            self.device,
            class_id.display()
        )
    }
}

pub fn read_u32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let mut output = Port::new(CONFIG_ADDRESS);
    let mut input = Port::new(CONFIG_DATA);

    let offset = offset & 0xFC;
    let address: u32 = 0x80000000
        | (u32::from(bus) << 16)
        | (u32::from(device) << 11)
        | (u32::from(function) << 8)
        | u32::from(offset);

    unsafe { output.write(address) };
    let reply: u32 = unsafe { input.read() };

    // reply >> ((offset & 2) * 8)
    reply
}

pub fn read_pci_device(bus: u8, device: u8, function: u8) -> Option<PciDevice> {
    let signature = read_u32(bus, device, function, 0);

    if signature == 0xffffffff {
        return None;
    }

    let class_dword = read_u32(bus, device, function, 8);

    let vendor: u16 = (signature & 0xffff) as u16;
    let device: u16 = ((signature >> 16) & 0xffff) as u16;
    let class = ((class_dword >> 24) & 0xff) as u8;

    Some(PciDevice {
        vendor,
        device,
        class,
    })
}

pub fn show() {
    for bus_id in 0..255 {
        for dev_id in 0..32 {
            for fun_id in 0..8 {
                let dev = read_pci_device(bus_id, dev_id, fun_id);
                match dev {
                    Some(dev) => {
                        serial_println!(
                            "{:02x}:{:02x}.{:x} {}",
                            bus_id,
                            dev_id,
                            fun_id,
                            dev.display()
                        );
                    }
                    None => (),
                }
            }
        }
    }
}
