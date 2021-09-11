use alloc::{format, string::String};

use super::database::{class_ids::get_class_name, vendor_ids::get_vendor_name};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDevice {
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
    pub subclass: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDeviceAddress {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDeviceBinding {
    pub address: PciDeviceAddress,
    pub device: PciDevice,
}

impl PciDevice {
    pub fn vendor_name(&self) -> Option<String> {
        get_vendor_name(self.vendor)
    }

    pub fn class_name(&self) -> Option<String> {
        get_class_name(self.class)
    }
}

impl PciDeviceBinding {
    pub fn display_line(&self) -> String {
        let class = match self.device.class_name() {
            Some(name) => name,
            None => format!("0x{:x}", self.device.class),
        };

        let vendor = match self.device.vendor_name() {
            Some(name) => name,
            None => format!("vendor=0x{:x}", self.device.vendor),
        };

        format!(
            "{:02x}:{:02x}.{:x} {}/0x{:x}: {}",
            self.address.bus,
            self.address.device,
            self.address.function,
            class,
            self.device.subclass,
            vendor,
        )
    }
}
