use x86_64::instructions::port::Port;

pub struct IoPorts {
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

    pub fn read_port0(&mut self) -> u32 {
        unsafe { self.port0.read() }
    }

    pub fn read_port1(&mut self) -> u32 {
        unsafe { self.port1.read() }
    }

    pub fn read_reset16(&mut self) -> u16 {
        unsafe { self.reset16.read() }
    }

    pub fn read_reset32(&mut self) -> u32 {
        unsafe { self.reset32.read() }
    }

    pub fn read_bcr32(&mut self, bcr_no: u32) -> u32 {
        self.write_rap32(bcr_no);
        unsafe { self.bcr32.read() }
    }

    pub fn write_bcr32(&mut self, bcr_no: u32, value: u32) {
        self.write_rap32(bcr_no);
        unsafe { self.bcr32.write(value) };
    }

    pub fn read_csr32(&mut self, csr_no: u32) -> u32 {
        self.write_rap32(csr_no);
        unsafe { self.csr32.read() }
    }

    pub fn write_csr32(&mut self, csr_no: u32, value: u32) {
        self.write_rap32(csr_no);
        unsafe { self.csr32.write(value) };
    }

    pub fn write_rap32(&mut self, value: u32) {
        unsafe { self.rap32.write(value) };
    }

    pub fn write_rdp32(&mut self, value: u32) {
        unsafe { self.rdp32.write(value) };
    }
}
