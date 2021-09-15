use crate::{print_all, println_all};

use crate::pci::model::PciDeviceBinding;
use crate::shortcuts::sleep;

use super::buffers::{BufferManager, NUM_RECEIVE_BUFFERS};
use super::ports::IoPorts;

pub struct PcNet {
    binding: PciDeviceBinding,
    io_ports: IoPorts,
    buffer_manager: BufferManager,
}

impl PcNet {
    pub fn initialize(binding: PciDeviceBinding, physical_memory_offset: u64) -> Self {
        let mut buffer_manager = BufferManager::new(physical_memory_offset);

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
        sleep(1 << 20);

        // Set 32bit mode
        io_ports.write_rdp32(0);

        // Set SWSTYLE to 2
        let mut csr_no = 58;
        let mut csr58 = io_ports.read_csr32(csr_no);
        csr58 &= 0xff00;
        csr58 |= 2;
        io_ports.write_csr32(csr_no, csr58);

        // Set ASEL bit
        let bcr_no = 2;
        let mut bcr2 = io_ports.read_bcr32(bcr_no);
        bcr2 |= 0x2;
        io_ports.write_bcr32(bcr_no, bcr2);

        // Set up buffers
        let mac = io_ports.read_mac_address();
        buffer_manager.initialize(mac);

        // Point the card to the init struct
        let is_addr = buffer_manager.address_of_init_struct();
        let low = is_addr & 0xffff;
        let high = (is_addr >> 16) & 0xffff;
        io_ports.write_csr32(1, low);
        io_ports.write_csr32(2, high);

        // Tell the card to initialize
        csr_no = 0;
        let mut csr0 = io_ports.read_csr32(csr_no);
        csr0 |= 0x1; // set bit 0
        io_ports.write_csr32(csr_no, csr0);
        println_all!("csr0: {:b}", csr0);

        // Poll waiting for card to initialize
        loop {
            csr0 = io_ports.read_csr32(csr_no);
            // wait for bit 8 to be set
            if csr0 & 0x80 > 0 {
                break;
            }
        }

        // Start the card
        csr0 = io_ports.read_csr32(csr_no);
        println_all!("csr0: {:b}", csr0);
        csr0 &= 0xfffffffa; // clear bits 0 and 2
        csr0 |= 0x00000002; // set bit 1
        io_ports.write_csr32(csr_no, csr0);

        csr0 = io_ports.read_csr32(csr_no);
        println_all!("csr0: {:b}", csr0);

        PcNet {
            binding,
            io_ports,
            buffer_manager,
        }
    }

    pub fn read_mac_address(&mut self) -> [u8; 6] {
        self.io_ports.read_mac_address()
    }

    pub fn poll_recv_packets(&self) {
        let bufman = &self.buffer_manager;

        loop {
            for idx in 0..NUM_RECEIVE_BUFFERS {
                let mut desc = bufman.receive_descriptors.entries[idx];
                let buffer = bufman.receive_buffers.buffers[idx];

                if desc.driver_has_ownership() {
                    print_all!("packet: {:?}", buffer.buffer);
                    desc.set_card_ownership();
                }
            }
        }
    }

    pub fn dump_phys_addresses(&self) {
        let bufman = &self.buffer_manager;

        println_all!("rx_bufs[31]: 0x{:x}", bufman.address_of_rx_buffer(31));
        println_all!("tx_bufs[7]: 0x{:x}", bufman.address_of_tx_buffer(7));

        println_all!("rx_desc[0]: 0x{:x}", bufman.address_of_rx_descriptor(0));
        println_all!("tx_desc[0]: 0x{:x}", bufman.address_of_tx_descriptor(0));

        println_all!("init_struct: 0x{:x}", bufman.address_of_init_struct());
    }
}
