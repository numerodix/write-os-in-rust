use crate::println_all;
use crate::serial_println;

use crate::pci::model::PciDeviceBinding;

use super::buffers::BufferManager;
use super::buffers::DescriptorEntry;
use super::ports::IoPorts;

pub struct PcNet {
    binding: PciDeviceBinding,
    io_ports: IoPorts,
    buffer_manager: BufferManager,

    rde: Option<*mut DescriptorEntry>,
    tde: Option<*mut DescriptorEntry>,

    rx_buffer_count: u16,
    tx_buffer_count: u16,
    buffer_size: u16,
}

impl PcNet {
    pub fn initialize(binding: PciDeviceBinding, physical_memory_offset: u64) -> Self {
        let buffer_manager = BufferManager::new(physical_memory_offset);

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

        println_all!(
            "rx_bufs[31]: 0x{:x}",
            buffer_manager.address_of_rx_buffer(31)
        );
        println_all!("tx_bufs[7]: 0x{:x}", buffer_manager.address_of_tx_buffer(7));

        PcNet {
            binding,
            io_ports,
            buffer_manager,
            rde: None,
            tde: None,
            rx_buffer_count: 0,
            tx_buffer_count: 0,
            buffer_size: 0,
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
