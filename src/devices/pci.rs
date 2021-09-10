use crate::serial_println;
use alloc::{format, string::String};
use enum_iterator::IntoEnumIterator;
use x86_64::instructions::port::Port;

pub const CONFIG_ADDRESS: u16 = 0xCF8;
pub const CONFIG_DATA: u16 = 0xCFC;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
#[repr(u16)]
pub enum PCI_VENDOR_ID {
    Intel = 0x8086,
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
pub struct PciDevice {
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
}

impl PciDevice {
    fn display(&self) -> String {
        let mut vendor_id = None;
        let mut class_id = None;

        for id in PCI_VENDOR_ID::into_enum_iter() {
            if self.vendor == id as u16 {
                vendor_id = Some(id);
            }
        }

        for id in PCI_DEVICE_CLASS::into_enum_iter() {
            if self.class == id as u8 {
                class_id = Some(id);
            }
        }

        let vendor_fmt = match vendor_id {
            Some(id) => format!("{:?}", id),
            None => format!("Unidentified(0x{:x})", self.vendor),
        };

        let class_fmt = match class_id {
            Some(id) => format!("{:?}", id),
            None => format!("Unidentified(0x{:x})", self.class),
        };

        format!(
            "PciDevice {{ vendor: {}, device: 0x{:x}, class: {} }}",
            vendor_fmt, self.device, class_fmt
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

pub fn read_pci_device(bus: u8, device: u8, function: u8) -> PciDevice {
    let signature = read_u32(bus, device, function, 0);
    let class_dword = read_u32(bus, device, function, 8);

    let vendor: u16 = (signature & 0xffff) as u16;
    let device: u16 = ((signature >> 16) & 0xffff) as u16;
    let class = ((class_dword >> 24) & 0xff) as u8;

    PciDevice {
        vendor,
        device,
        class,
    }
}

pub fn show() {
    for bus_id in 0..1 {
        for dev_id in 0..4 {
            // let mut offset = 0;
            // let val = read_dword(bus_id, dev_id, 0, offset);
            // serial_println!("bus: {}  dev: {}  offset: {}  got: 0x{:x}", bus_id, dev_id, offset, val);

            // offset = 4;
            // let val = read_dword(bus_id, dev_id, 0, offset);
            // serial_println!("bus: {}  dev: {}  offset: {}  got: 0x{:x}", bus_id, dev_id, offset, val);

            // let offset = 8;
            // let val = read_dword(bus_id, dev_id, 0, offset);
            // serial_println!("bus: {}  dev: {}  offset: {}  got: 0x{:x}", bus_id, dev_id, offset, val);

            // serial_println!();

            let dev = read_pci_device(bus_id, dev_id, 0);
            serial_println!(
                "bus: {}  dev: {}  ::  vendor: 0x{:x}  device: 0x{:x}  class: 0x{:x}",
                bus_id,
                dev_id,
                dev.vendor,
                dev.device,
                dev.class,
            );
            serial_println!("bus: {}  dev: {}  ::  {}", bus_id, dev_id, dev.display());
        }
    }
}
