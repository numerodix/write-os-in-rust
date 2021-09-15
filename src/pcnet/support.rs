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

        let phys = addr - self.physical_memory_offset;

        assert!(phys < (1 << 31));

        phys as u32
    }
}
