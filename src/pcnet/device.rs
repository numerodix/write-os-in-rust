use crate::{allocator::ALLOCATOR, serial_println};
use alloc::alloc::GlobalAlloc;
use core::{alloc::Layout, mem};

use crate::pci::model::PciDeviceBinding;

use super::buffers::DescriptorEntry;
use super::buffers::ReceiveBuffers;
use super::ports::IoPorts;

pub struct PcNet {
    binding: PciDeviceBinding,
    io_ports: IoPorts,

    rde: Option<*mut DescriptorEntry>,
    tde: Option<*mut DescriptorEntry>,

    rx_buffers: Option<*mut u64>,
    tx_buffers: Option<*mut u64>,

    rx_buffer_count: u16,
    tx_buffer_count: u16,
    buffer_size: u16,

    physical_memory_offset: u64,
}

impl PcNet {
    pub fn initialize(binding: PciDeviceBinding, phyical_memory_offset: u64) -> Self {
        // Enable io ports and bus mastering of the card
        let offset = 4;
        let mut conf = binding.config_read(offset);
        conf &= 0xffff0000; // clear command register, preserve status register
        conf |= 0x5; // set bits 0 and 2
        binding.config_write(offset, conf);

        // Populate io_base
        let io_base = (binding.device.bar0 & 0xfffffffc) as u16;
        let mut io_ports = IoPorts::new(io_base);

        // Reset the card
        io_ports.read_reset32();
        io_ports.read_reset16();

        // wait 1us (sort of)
        Self::sleep(1 << 20);

        // Set 32bit mode
        io_ports.write_rdp32(0);

        // Set SWSTYLE to 2
        let csr_no = 58;
        let mut csr58 = io_ports.read_csr32(csr_no);
        csr58 &= 0xff00;
        csr58 |= 2;
        io_ports.write_csr32(csr_no, csr58);

        // Set ASEL bit
        let bcr_no = 2;
        let mut bcr2 = io_ports.read_bcr32(bcr_no);
        bcr2 |= 0x2;
        io_ports.write_bcr32(bcr_no, bcr2);

        let size = ReceiveBuffers::size();
        let layout = Layout::from_size_align(size, 32).unwrap();
        let ptr = unsafe { ALLOCATOR.alloc(layout) };
        serial_println!("add: {:?}", ptr);

        PcNet {
            binding,
            io_ports,
            rde: None,
            tde: None,
            rx_buffers: None,
            tx_buffers: None,
            rx_buffer_count: 0,
            tx_buffer_count: 0,
            buffer_size: 0,
            physical_memory_offset: phyical_memory_offset,
        }
    }

    fn sleep(cycles: u64) {
        let mut sum = 0;
        for i in 0..cycles {
            sum += i;
        }
    }

    pub fn read_mac_address(&mut self) -> [u8; 6] {
        let fst_byte: u32 = self.io_ports.read_port0();
        let snd_byte: u32 = self.io_ports.read_port1();

        let mut mac = [0u8; 6];
        mac[0] = (fst_byte & 0xff) as u8;
        mac[1] = ((fst_byte >> 8) & 0xff) as u8;
        mac[2] = ((fst_byte >> 16) & 0xff) as u8;
        mac[3] = ((fst_byte >> 24) & 0xff) as u8;
        mac[4] = (snd_byte & 0xff) as u8;
        mac[5] = ((snd_byte >> 8) & 0xff) as u8;

        mac
    }
}
