use crate::pci::model::{PciDeviceAddress, PciDeviceBinding};

use super::comms::config_read;
use super::model::PciDevice;
use alloc::vec;
use alloc::vec::Vec;

pub fn detect_device(bus: u8, device: u8, function: u8) -> Option<PciDevice> {
    let register_0 = config_read(bus, device, function, 0);

    if register_0 == 0xffffffff {
        return None;
    }

    let register_1 = config_read(bus, device, function, 4);
    let register_2 = config_read(bus, device, function, 8);
    let register_3 = config_read(bus, device, function, 12);
    let bar0 = config_read(bus, device, function, 16);
    let bar1 = config_read(bus, device, function, 20);
    let bar2 = config_read(bus, device, function, 24);
    let bar3 = config_read(bus, device, function, 28);
    let bar4 = config_read(bus, device, function, 32);
    let bar5 = config_read(bus, device, function, 36);
    let register_15 = config_read(bus, device, function, 60);

    let vendor: u16 = (register_0 & 0xffff) as u16;
    let device: u16 = ((register_0 >> 16) & 0xffff) as u16;
    let command: u16 = (register_1 & 0xffff) as u16;
    let status: u16 = ((register_1 >> 16) & 0xffff) as u16;
    let class = ((register_2 >> 24) & 0xff) as u8;
    let subclass = ((register_2 >> 16) & 0xff) as u8;
    let prog_if = ((register_2 >> 8) & 0xff) as u8;
    let revision = (register_2 & 0xff) as u8;
    let header_type = ((register_3 >> 16) & 0xff) as u8;
    let interrupt_pin = (register_15 >> 16 & 0xff) as u8;
    let interrupt_line = (register_15 >> 24 & 0xff) as u8;

    Some(PciDevice {
        vendor,
        device,
        command,
        status,
        class,
        subclass,
        prog_if,
        revision,
        header_type,
        bar0,
        bar1,
        bar2,
        bar3,
        bar4,
        bar5,
        interrupt_pin,
        interrupt_line,
    })
}

pub fn detect_all_devices() -> Vec<PciDeviceBinding> {
    let mut bindings = vec![];

    for bus_id in 0..255 {
        for dev_id in 0..32 {
            for fun_id in 0..8 {
                if let Some(device) = detect_device(bus_id, dev_id, fun_id) {
                    let address = PciDeviceAddress {
                        bus: bus_id,
                        device: dev_id,
                        function: fun_id,
                    };
                    let binding = PciDeviceBinding { address, device };
                    bindings.push(binding);
                }
            }
        }
    }

    bindings
}
