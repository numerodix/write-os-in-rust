use x86_64::instructions::port::Port;

pub struct IoPorts {
    io_base: u16,

    port0: Port<u16>,
    port1: Port<u16>,
    port2: Port<u16>,

    bcr16: Port<u16>,
    csr16: Port<u16>,
    rap16: Port<u16>,
    rdp16: Port<u16>,
    reset16: Port<u16>,

    reset32: Port<u32>,
}

impl IoPorts {
    pub fn new(io_base: u16) -> Self {
        IoPorts {
            io_base,

            port0: Port::new(io_base),
            port1: Port::new(io_base + 0x02),
            port2: Port::new(io_base + 0x04),

            csr16: Port::new(io_base + 0x10),
            rdp16: Port::new(io_base + 0x10),
            rap16: Port::new(io_base + 0x12),
            reset16: Port::new(io_base + 0x14),
            bcr16: Port::new(io_base + 0x16),

            reset32: Port::new(io_base + 0x18),
        }
    }

    pub fn read_port0(&mut self) -> u16 {
        unsafe { self.port0.read() }
    }

    pub fn read_port1(&mut self) -> u16 {
        unsafe { self.port1.read() }
    }

    pub fn read_port2(&mut self) -> u16 {
        unsafe { self.port2.read() }
    }

    pub fn read_reset16(&mut self) -> u16 {
        unsafe { self.reset16.read() }
    }

    pub fn read_reset32(&mut self) -> u32 {
        unsafe { self.reset32.read() }
    }

    pub fn read_bcr16(&mut self, bcr_no: u16) -> u16 {
        self.write_rap16(bcr_no);
        unsafe { self.bcr16.read() }
    }

    pub fn write_bcr16(&mut self, bcr_no: u16, value: u16) {
        self.write_rap16(bcr_no);
        unsafe { self.bcr16.write(value) };
    }

    pub fn read_csr16(&mut self, csr_no: u16) -> u16 {
        self.write_rap16(csr_no);
        unsafe { self.csr16.read() }
    }

    pub fn write_csr16(&mut self, csr_no: u16, value: u16) {
        self.write_rap16(csr_no);
        unsafe { self.csr16.write(value) };
    }

    pub fn write_rap16(&mut self, value: u16) {
        unsafe { self.rap16.write(value) };
    }

    pub fn write_rdp16(&mut self, value: u16) {
        unsafe { self.rdp16.write(value) };
    }

    pub fn read_mac_address(&mut self) -> [u8; 6] {
        let fst_word: u16 = self.read_port0();
        let snd_word: u16 = self.read_port1();
        let thd_word: u16 = self.read_port2();

        let mut mac = [0u8; 6];
        mac[0] = (fst_word & 0xff) as u8;
        mac[1] = ((fst_word >> 8) & 0xff) as u8;
        mac[2] = ((snd_word >> 0) & 0xff) as u8;
        mac[3] = ((snd_word >> 8) & 0xff) as u8;
        mac[4] = (thd_word & 0xff) as u8;
        mac[5] = ((thd_word >> 8) & 0xff) as u8;

        mac
    }
}
