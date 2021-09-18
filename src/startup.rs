use bootloader::BootInfo;

use crate::pci::detect::detect_all_devices;
use crate::pci::model::PciDeviceBinding;
use crate::pcnet::PcNet;
use crate::println_all;
use crate::shortcuts::format_mac_address;

pub fn init_pcnet_card(binding: PciDeviceBinding) {
    let page_mapping_offset: u64 = 0x4444441c0000;

    println_all!("pcnet32: Initializing pcnet card");
    let mut pcnet = PcNet::initialize(binding, page_mapping_offset);

    let mac = pcnet.read_mac_address();
    println_all!("pcnet32: Using mac address: {}", format_mac_address(mac));
    // pcnet.dump_phys_addresses();

    println_all!("pcnet32: Polling for incoming packets...");
    pcnet.poll_recv_packets();
}

pub fn init_pci_devices(boot_info: &'static BootInfo) {
    println_all!("pci: Detecting PCI devices...");
    let devices = detect_all_devices();

    for device in devices.iter() {
        let line = device.display_line();
        println_all!("pci: {}", line);

        let lines = device.display_block();
        for line in lines {
            println_all!("pci: {}", line);
        }
    }

    let mut binding = devices[devices.len() - 1];
    init_pcnet_card(binding);

    // binding.redetect();
    // let lines = binding.display_block();
    // for line in lines {
    //     println_all!("pci: {}", line);
    // }
}
