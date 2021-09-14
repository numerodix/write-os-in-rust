use crate::serial_println;

pub struct AddrTranslator {
    physical_memory_offset: u64,
}

impl AddrTranslator {
    pub fn new(physical_memory_offset: u64) -> Self {
        Self {
            physical_memory_offset,
        }
    }

    pub fn translate(&self, addr: u64) -> u32 {
        assert!(addr >= self.physical_memory_offset);

        serial_println!("offset: 0x{:x}", self.physical_memory_offset);
        serial_println!("addr:   0x{:x}", addr);
        let phys = addr - self.physical_memory_offset;
        serial_println!("phys:   0x{:x}", phys);

        assert!(phys < (1 << 31));

        phys as u32
    }
}
