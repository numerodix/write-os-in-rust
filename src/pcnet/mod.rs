use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

use crate::pci::model::PciDeviceBinding;

pub struct PcNet {
    pub binding: PciDeviceBinding,
    pub io_base: u16,

    bcr32: Option<Port<u32>>,
    csr32: Option<Port<u32>>,
    rap32: Option<Port<u32>>,
    rdp32: Option<Port<u32>>,
    reset32: Option<Port<u32>>,
}

impl PcNet {
    pub fn new(binding: PciDeviceBinding) -> Self {
        PcNet { binding, io_base: 0, bcr32: None, csr32: None, rap32: None, rdp32: None, reset32: None }
    }

    fn read_bcr32(&mut self, bcr_no: u32) -> u32 {
        self.write_rap32(bcr_no);
        unsafe { self.bcr32.as_mut().unwrap().read() }
    }

    fn write_bcr32(&mut self, bcr_no: u32, value: u32) {
        self.write_rap32(bcr_no);
        unsafe { self.bcr32.as_mut().unwrap().write(value) };
    }

    fn read_csr32(&mut self, csr_no: u32) -> u32 {
        self.write_rap32(csr_no);
        unsafe { self.csr32.as_mut().unwrap().read() }
    }

    fn write_csr32(&mut self, csr_no: u32, value: u32) {
        self.write_rap32(csr_no);
        unsafe { self.csr32.as_mut().unwrap().write(value) };
    }

    fn write_rap32(&mut self, value: u32) {
        unsafe { self.rap32.as_mut().unwrap().write(value) };
    }

    pub fn init(&mut self) {
        // Enable io ports and bus mastering of the card
        let offset = 4;
        let mut conf = self.binding.config_read(offset);
        conf &= 0xffff0000; // clear command register, preserve status register
        conf |= 0x5; // set bits 0 and 2
        self.binding.config_write(offset, conf);

        // Populate io_base
        self.io_base = (self.binding.device.bar0 & 0xfffffffc) as u16;

        // Instantiate Port data structures
        self.rdp32 = Some(Port::new(self.io_base + 0x10));
        self.csr32 = Some(Port::new(self.io_base + 0x10));
        self.rap32 = Some(Port::new(self.io_base + 0x14));
        self.reset32 = Some(Port::new(self.io_base + 0x18));
        self.bcr32 = Some(Port::new(self.io_base + 0x1c));

        // Reset the card
        let mut reset_reg_16bit: PortGeneric<u16, ReadWriteAccess> = Port::new(self.io_base + 0x14);

        unsafe { self.reset32.as_mut().unwrap().read() };
        unsafe { reset_reg_16bit.read() };

        // wait 1us

        // Set 32bit mode
        unsafe { self.rdp32.as_mut().unwrap().write(0) };

        // Set SWSTYLE to 2
        let csr_no = 58;
        let mut csr58 = self.read_csr32(csr_no);
        csr58 &= 0xff00;
        csr58 |= 2;
        self.write_csr32(csr_no, csr58);

        // Set ASEL bit
        let bcr_no = 2;
        let mut bcr2 = self.read_bcr32(bcr_no);
        bcr2 |= 0x2;
        self.write_bcr32(bcr_no, bcr2);
    }
}
