use bootloader::BootInfo;

use crate::pci::detect::detect_all_devices;
use crate::pcnet::PcNet;
use crate::println_all;
use crate::shortcuts::format_mac_address;

pub fn init_pci_devices(boot_info: &'static BootInfo) {
    println_all!("pci: Detecting PCI devices...");
    let devices = detect_all_devices();

    // use bar1 as a mem location and read its value
    // let bar1 = devices[devices.len() - 1].device.bar1;
    // let ptr = (bar1 as u64 + boot_info.physical_memory_offset) as *const u64;
    // let v = unsafe { *ptr };
    // serial_println!("bar1: 0x{:x}", v);

    for device in devices.iter() {
        let line = device.display_line();
        println_all!("pci: {}", line);

        let lines = device.display_block();
        for line in lines {
            println_all!("pci: {}", line);
        }
    }

    // why is this not boot_info.physical_memory_offset -NOR- HEAP_START??
    let page_mapping_offset: u64 = 0x4444441c0000;

    println_all!("init pcnet card");
    let mut binding = devices[devices.len() - 1];
    let mut pcnet = PcNet::initialize(binding, page_mapping_offset);

    let mac = pcnet.read_mac_address();
    println_all!("mac: {}", format_mac_address(mac));

    binding.redetect();
    let lines = binding.display_block();
    for line in lines {
        println_all!("pci: {}", line);
    }
}
