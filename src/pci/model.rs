use core::fmt::LowerHex;

use alloc::borrow::ToOwned;
use alloc::{format, string::String};

use super::database::subclass_ids::get_subclass_name;
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
    pub fn vendor_name(&self) -> Option<&'static str> {
        get_vendor_name(self.vendor)
    }

    pub fn class_name(&self) -> Option<&'static str> {
        get_class_name(self.class)
    }

    pub fn subclass_name(&self) -> Option<&'static str> {
        get_subclass_name(self.class, self.subclass)
    }
}

impl PciDeviceBinding {
    pub fn display_line(&self) -> String {
        let class = match self.device.class_name() {
            Some(name) => name.to_owned(),
            None => format!("0x{:x}", self.device.class),
        };

        let subclass = match self.device.subclass_name() {
            Some(name) => name.to_owned(),
            None => format!("0x{:x}", self.device.subclass),
        };

        let vendor = match self.device.vendor_name() {
            Some(name) => name.to_owned(),
            None => format!("vendor=0x{:x}", self.device.vendor),
        };

        format!(
            "{:02x}:{:02x}.{:x} {}/{}: {}",
            self.address.bus, self.address.device, self.address.function, class, subclass, vendor,
        )
    }

    pub fn name_or_hex<N: LowerHex>(&self, name: Option<&'static str>, number: N) -> String {
        match name {
            Some(name) => name.to_owned(),
            None => format!("0x{:x}", number),
        }
    }

    pub fn display_block(&self) -> String {
        let prefix = format!(
            "{:02x}:{:02x}.{:x}  ",
            self.address.bus, self.address.device, self.address.function
        );

        let vendor = format!(
            "vendor: {}",
            self.name_or_hex(self.device.vendor_name(), self.device.vendor)
        );

        let class = format!(
            "class: {}",
            self.name_or_hex(self.device.class_name(), self.device.class)
        );

        let subclass = format!(
            "subclass: {}",
            self.name_or_hex(self.device.subclass_name(), self.device.subclass)
        );

        format!(
            "{}{}\n{}{}\n{}{}\n",
            prefix, vendor, prefix, class, prefix, subclass
        )
    }
}
