use crate::pci::model::{PciDeviceAddress, PciDeviceBinding};

use super::model::PciDevice;
use alloc::vec;
use alloc::vec::Vec;
use x86_64::instructions::port::Port;

pub const CONFIG_ADDRESS: u16 = 0xCF8;
pub const CONFIG_DATA: u16 = 0xCFC;

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

    reply
}

pub fn detect_device(bus: u8, device: u8, function: u8) -> Option<PciDevice> {
    let signature = read_u32(bus, device, function, 0);

    if signature == 0xffffffff {
        return None;
    }

    let class_dword = read_u32(bus, device, function, 8);
    let bar0 = read_u32(bus, device, function, 16);
    let bar1 = read_u32(bus, device, function, 20);
    let bar2 = read_u32(bus, device, function, 24);
    let bar3 = read_u32(bus, device, function, 28);
    let bar4 = read_u32(bus, device, function, 32);
    let bar5 = read_u32(bus, device, function, 36);

    let vendor: u16 = (signature & 0xffff) as u16;
    let device: u16 = ((signature >> 16) & 0xffff) as u16;
    let class = ((class_dword >> 24) & 0xff) as u8;
    let subclass = ((class_dword >> 16) & 0xff) as u8;
    let prog_if = ((class_dword >> 8) & 0xff) as u8;
    let revision = (class_dword & 0xff) as u8;

    Some(PciDevice {
        vendor,
        device,
        class,
        subclass,
        prog_if,
        revision,
        bar0,
        bar1,
        bar2,
        bar3,
        bar4,
        bar5,
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
