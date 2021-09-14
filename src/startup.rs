use alloc::format;
use bootloader::BootInfo;

use crate::pci::detect::detect_all_devices;
use crate::pcnet::PcNet;
use crate::serial_println;
use crate::shortcuts::format_mac_address;
use crate::shortcuts::print_both;
use crate::shortcuts::println_both;

pub fn init_pci_devices(boot_info: &'static BootInfo) {
    println_both("pci: Detecting PCI devices...");
    let devices = detect_all_devices();

    // use bar1 as a mem location and read its value
    // let bar1 = devices[devices.len() - 1].device.bar1;
    // let ptr = (bar1 as u64 + boot_info.physical_memory_offset) as *const u64;
    // let v = unsafe { *ptr };
    // serial_println!("bar1: 0x{:x}", v);

    for device in devices.iter() {
        let line = device.display_line();
        let line = format!("pci: {}", line);
        println_both(&line);

        let lines = device.display_block();
        for line in lines {
            let line = format!("pci: {}\n", line);
            print_both(&line);
        }
    }

    println_both("init pcnet card");
    let mut binding = devices[devices.len() - 1];
    let pcnet = PcNet::initialize(binding, boot_info.physical_memory_offset);

    let mac = pcnet.read_mac_address();
    println_both(&format!("mac: {}", format_mac_address(mac)));

    binding.redetect();
    let lines = binding.display_block();
    for line in lines {
        let line = format!("pci: {}\n", line);
        print_both(&line);
    }
}
