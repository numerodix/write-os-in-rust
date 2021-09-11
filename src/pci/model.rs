use core::fmt::LowerHex;

use alloc::borrow::ToOwned;
use alloc::{format, string::String};

use super::database::device_ids::get_device_name;
use super::database::prog_if_ids::get_prog_if_name;
use super::database::subclass_ids::get_subclass_name;
use super::database::{class_ids::get_class_name, vendor_ids::get_vendor_name};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDevice {
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub bar0: u32,
    pub bar1: u32,
    pub bar2: u32,
    pub bar3: u32,
    pub bar4: u32,
    pub bar5: u32,
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

    pub fn device_name(&self) -> Option<&'static str> {
        get_device_name(self.vendor, self.device)
    }

    pub fn class_name(&self) -> Option<&'static str> {
        get_class_name(self.class)
    }

    pub fn subclass_name(&self) -> Option<&'static str> {
        get_subclass_name(self.class, self.subclass)
    }

    pub fn prog_if_name(&self) -> Option<&'static str> {
        get_prog_if_name(self.class, self.subclass, self.prog_if)
    }
}

impl PciDeviceBinding {
    pub fn display_line(&self) -> String {
        let class = format!(
            "{}",
            self.name_or_hex(self.device.class_name(), self.device.class)
        );
        let device = format!(
            "{}",
            self.name_or_hex(self.device.device_name(), self.device.device)
        );

        format!(
            "{:02x}:{:02x}.{:x} {}: {}",
            self.address.bus, self.address.device, self.address.function, class, device,
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

        let signature = format!(
            "signature: {} {}",
            self.name_or_hex(None, self.device.vendor),
            self.name_or_hex(None, self.device.device),
        );

        let device = format!(
            "device: {}",
            self.name_or_hex(self.device.device_name(), self.device.device)
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

        let prog_if = format!(
            "prog_if: {}",
            self.name_or_hex(self.device.prog_if_name(), self.device.prog_if)
        );

        let revision = format!("revision: {}", self.name_or_hex(None, self.device.revision));

        let bar0 = format!("bar0: {}", self.name_or_hex(None, self.device.bar0));
        let bar1 = format!("bar1: {}", self.name_or_hex(None, self.device.bar1));
        let bar2 = format!("bar2: {}", self.name_or_hex(None, self.device.bar2));
        let bar3 = format!("bar3: {}", self.name_or_hex(None, self.device.bar3));
        let bar4 = format!("bar4: {}", self.name_or_hex(None, self.device.bar4));
        let bar5 = format!("bar5: {}", self.name_or_hex(None, self.device.bar5));

        format!(
            "{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n",
            prefix,
            vendor,
            prefix,
            device,
            prefix,
            signature,
            prefix,
            class,
            prefix,
            subclass,
            prefix,
            prog_if,
            prefix,
            revision,
            prefix,
            bar0,
            prefix,
            bar1,
            prefix,
            bar2,
            prefix,
            bar3,
            prefix,
            bar4,
            prefix,
            bar5,
        )
    }
}
