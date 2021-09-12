use core::fmt::LowerHex;

use alloc::borrow::ToOwned;
use alloc::vec;
use alloc::vec::Vec;
use alloc::{format, string::String};

use super::comms::{config_read, config_write};
use super::database::device_ids::get_device_name;
use super::database::prog_if_ids::get_prog_if_name;
use super::database::subclass_ids::get_subclass_name;
use super::database::{class_ids::get_class_name, vendor_ids::get_vendor_name};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDevice {
    pub vendor: u16,
    pub device: u16,
    pub status: u16,
    pub command: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub header_type: u8,
    pub bar0: u32,
    pub bar1: u32,
    pub bar2: u32,
    pub bar3: u32,
    pub bar4: u32,
    pub bar5: u32,
    pub interrupt_pin: u8,
    pub interrupt_line: u8,
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
    pub fn config_read(&self, offset: u8) -> u32 {
        config_read(
            self.address.bus,
            self.address.device,
            self.address.function,
            offset,
        )
    }

    pub fn config_write(&self, offset: u8, value: u32) {
        config_write(
            self.address.bus,
            self.address.device,
            self.address.function,
            offset,
            value,
        )
    }

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

    fn name_or_hex<N: LowerHex>(&self, name: Option<&'static str>, number: N) -> String {
        match name {
            Some(name) => name.to_owned(),
            None => format!("0x{:x}", number),
        }
    }

    pub fn display_block(&self) -> Vec<String> {
        let mut lines = vec![];

        let prefix = format!(
            "{:02x}:{:02x}.{:x}  ",
            self.address.bus, self.address.device, self.address.function
        );

        let vendor = format!(
            "{}vendor: {}",
            prefix,
            self.name_or_hex(self.device.vendor_name(), self.device.vendor)
        );
        lines.push(vendor);

        let device = format!(
            "{}device: {}",
            prefix,
            self.name_or_hex(self.device.device_name(), self.device.device)
        );
        lines.push(device);

        let signature = format!(
            "{}signature: {} {}",
            prefix,
            self.name_or_hex(None, self.device.vendor),
            self.name_or_hex(None, self.device.device),
        );
        lines.push(signature);

        let class = format!(
            "{}class: {}",
            prefix,
            self.name_or_hex(self.device.class_name(), self.device.class)
        );
        lines.push(class);

        let subclass = format!(
            "{}subclass: {}",
            prefix,
            self.name_or_hex(self.device.subclass_name(), self.device.subclass)
        );
        lines.push(subclass);

        let prog_if = format!(
            "{}prog_if: {}",
            prefix,
            self.name_or_hex(self.device.prog_if_name(), self.device.prog_if)
        );
        lines.push(prog_if);

        let revision = format!(
            "{}revision: {}",
            prefix,
            self.name_or_hex(None, self.device.revision)
        );
        lines.push(revision);

        let header_type = format!(
            "{}header_type: {}",
            prefix,
            self.name_or_hex(None, self.device.header_type)
        );
        lines.push(header_type);

        let status = format!(
            "{}status: {}",
            prefix,
            self.name_or_hex(None, self.device.status)
        );
        lines.push(status);

        let command = format!(
            "{}command: {}",
            prefix,
            self.name_or_hex(None, self.device.command)
        );
        lines.push(command);

        let bar0 = format!(
            "{}bar0: {}",
            prefix,
            self.name_or_hex(None, self.device.bar0)
        );
        let bar1 = format!(
            "{}bar1: {}",
            prefix,
            self.name_or_hex(None, self.device.bar1)
        );
        let bar2 = format!(
            "{}bar2: {}",
            prefix,
            self.name_or_hex(None, self.device.bar2)
        );
        let bar3 = format!(
            "{}bar3: {}",
            prefix,
            self.name_or_hex(None, self.device.bar3)
        );
        let bar4 = format!(
            "{}bar4: {}",
            prefix,
            self.name_or_hex(None, self.device.bar4)
        );
        let bar5 = format!(
            "{}bar5: {}",
            prefix,
            self.name_or_hex(None, self.device.bar5)
        );
        lines.push(bar0);
        lines.push(bar1);
        lines.push(bar2);
        lines.push(bar3);
        lines.push(bar4);
        lines.push(bar5);

        let interrupt_pin = format!(
            "{}interrupt_pin: {}",
            prefix,
            self.name_or_hex(None, self.device.interrupt_pin)
        );
        lines.push(interrupt_pin);

        let interrupt_line = format!(
            "{}interrupt_line: {}",
            prefix,
            self.name_or_hex(None, self.device.interrupt_line)
        );
        lines.push(interrupt_line);

        lines
    }
}
