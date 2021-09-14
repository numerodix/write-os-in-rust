use core::{alloc::Layout, intrinsics::size_of};

use x86_64::instructions::port::Port;

use crate::pci::model::PciDeviceBinding;

struct IoPorts {
    io_base: u16,

    port0: Port<u32>,
    port1: Port<u32>,

    bcr32: Port<u32>,
    csr32: Port<u32>,
    rap32: Port<u32>,
    rdp32: Port<u32>,
    reset32: Port<u32>,

    reset16: Port<u16>,
}

impl IoPorts {
    pub fn new(io_base: u16) -> Self {
        IoPorts {
            io_base,

            port0: Port::new(io_base),
            port1: Port::new(io_base + 0x04),

            csr32: Port::new(io_base + 0x10),
            rdp32: Port::new(io_base + 0x10),
            rap32: Port::new(io_base + 0x14),
            reset32: Port::new(io_base + 0x18),
            bcr32: Port::new(io_base + 0x1c),

            reset16: Port::new(io_base + 0x14),
        }
    }

    fn read_port0(&mut self) -> u32 {
        unsafe { self.port0.read() }
    }

    fn read_port1(&mut self) -> u32 {
        unsafe { self.port1.read() }
    }

    fn read_reset16(&mut self) -> u16 {
        unsafe { self.reset16.read() }
    }

    fn read_reset32(&mut self) -> u32 {
        unsafe { self.reset32.read() }
    }

    fn read_bcr32(&mut self, bcr_no: u32) -> u32 {
        self.write_rap32(bcr_no);
        unsafe { self.bcr32.read() }
    }

    fn write_bcr32(&mut self, bcr_no: u32, value: u32) {
        self.write_rap32(bcr_no);
        unsafe { self.bcr32.write(value) };
    }

    fn read_csr32(&mut self, csr_no: u32) -> u32 {
        self.write_rap32(csr_no);
        unsafe { self.csr32.read() }
    }

    fn write_csr32(&mut self, csr_no: u32, value: u32) {
        self.write_rap32(csr_no);
        unsafe { self.csr32.write(value) };
    }

    fn write_rap32(&mut self, value: u32) {
        unsafe { self.rap32.write(value) };
    }

    fn write_rdp32(&mut self, value: u32) {
        unsafe { self.rdp32.write(value) };
    }
}

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

#[repr(packed)]
struct DescriptorEntry {
    buffer_address: u32, // 4 bytes
    count: u16,          // 2 bytes
    unused1: u8,         // 1 byte
    ownership: u8,       // 1 byte
    unused2: u32,        // 4 bytes
    unused3: u32,        // 4 bytes
}

#[repr(packed)]
struct PacketBuffer {
    buffer: [u8; 1520],
}

#[repr(packed)]
struct ReceiveBuffers {
    buffers: [PacketBuffer; 32],
}

#[repr(packed)]
struct TransmitBuffers {
    buffers: [PacketBuffer; 8],
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
