use alloc::format;

use crate::pci::detect::detect_all_devices;
use crate::shortcuts::println_both;

pub fn init_pci_devices() {
    println_both("pci: Detecting PCI devices...");
    let devices = detect_all_devices();

    for device in devices {
        let line = device.display_line();
        let line = format!("pci: {}", line);
        println_both(&line);
    }
}
