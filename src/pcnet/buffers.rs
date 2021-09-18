use core::mem::size_of;

use alloc::boxed::Box;

use super::support::AddrTranslator;

pub const NUM_RECEIVE_BUFFERS: usize = 32;
pub const NUM_TRANSMIT_BUFFERS: usize = 8;
pub const BUFFER_SIZE: usize = 1520;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct DescriptorEntry {
    buffer_address: u32, // 4 bytes
    size: u16,           // 2 bytes
    unused1: u8,         // 1 byte
    ownership: u8,       // 1 byte
    unused2: u32,        // 4 bytes
    unused3: u32,        // 4 bytes
}

impl DescriptorEntry {
    fn set_size(&mut self, size: usize) {
        assert!(size <= 1 << 16);

        let mut sz = !(size as u16);
        sz &= 0x0fff;
        sz |= 0xf000;
        self.size = sz;
    }

    pub fn driver_has_ownership(&mut self) -> bool {
        if self.ownership & 0x80 > 0 {
            return true;
        }
        return false;
    }

    pub fn set_card_ownership(&mut self) {
        self.ownership = 0x80;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct DescriptorRingBuffer<const S: usize> {
    pub entries: [DescriptorEntry; S],
}

impl<const S: usize> DescriptorRingBuffer<S> {
    fn new() -> Self {
        Self {
            entries: [DescriptorEntry {
                buffer_address: 0,
                size: 0,
                unused1: 0,
                ownership: 0,
                unused2: 0,
                unused3: 0,
            }; S],
        }
    }

    pub fn len(&self) -> usize {
        S
    }
}

type ReceiveDescriptors = DescriptorRingBuffer<NUM_RECEIVE_BUFFERS>;
type TransmitDescriptors = DescriptorRingBuffer<NUM_TRANSMIT_BUFFERS>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct PacketBuffer {
    pub buffer: [u8; BUFFER_SIZE],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct PacketRingBuffer<const S: usize> {
    pub buffers: [PacketBuffer; S],
}

impl<const S: usize> PacketRingBuffer<S> {
    fn new() -> Self {
        Self {
            buffers: [PacketBuffer {
                buffer: [0; BUFFER_SIZE],
            }; S],
        }
    }

    pub fn len(&self) -> usize {
        S
    }
}

type ReceiveBuffers = PacketRingBuffer<NUM_RECEIVE_BUFFERS>;
type TransmitBuffers = PacketRingBuffer<NUM_TRANSMIT_BUFFERS>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct InitBlock {
    // offset: 0
    mode: u16,
    rlen: u8,
    tlen: u8,

    // offset: 4
    mac0: u8,
    mac1: u8,
    mac2: u8,
    mac3: u8,

    // offset: 8
    mac4: u8,
    mac5: u8,
    reserved: u16,

    // offset: 12
    unused: u64, // really ladr but we don't use it

    // offset: 20
    rx_desc_phys_addr: u32,

    // offset: 24
    tx_desc_phys_addr: u32,
}

impl InitBlock {
    fn new() -> Self {
        Self {
            mode: 0,
            rlen: 0,
            tlen: 0,
            mac0: 0,
            mac1: 0,
            mac2: 0,
            mac3: 0,
            mac4: 0,
            mac5: 0,
            reserved: 0,
            unused: 0,
            rx_desc_phys_addr: 0,
            tx_desc_phys_addr: 0,
        }
    }

    fn set_rlen(&mut self, rlen: u16) {
        // NUM_RECEIVE_BUFFERS: 32 ; 2^5 = 32
        let byte: u8 = 5 << 4;
    }

    fn set_tlen(&mut self, tlen: u16) {
        // NUM_TRANSMIT_BUFFERS: 8 ; 2^3 = 8
        let byte: u8 = 3 << 4;
    }
}

pub struct BufferManager {
    translator: AddrTranslator,

    pub receive_buffers: Box<ReceiveBuffers>,
    pub transmit_buffers: Box<TransmitBuffers>,

    pub receive_descriptors: Box<ReceiveDescriptors>,
    pub transmit_descriptors: Box<TransmitDescriptors>,

    init_block: Box<InitBlock>,
}

impl BufferManager {
    pub fn new(physical_memory_offset: u64) -> Self {
        Self {
            translator: AddrTranslator::new(physical_memory_offset),

            receive_buffers: Box::new(ReceiveBuffers::new()),
            transmit_buffers: Box::new(TransmitBuffers::new()),

            receive_descriptors: Box::new(ReceiveDescriptors::new()),
            transmit_descriptors: Box::new(TransmitDescriptors::new()),

            init_block: Box::new(InitBlock::new()),
        }
    }

    pub fn address_of_rx_descriptor(&self, idx: usize) -> u32 {
        assert!(idx < self.receive_descriptors.len());

        let base_addr = &*self.receive_descriptors as *const ReceiveDescriptors as u64;
        let addr = base_addr + (idx * size_of::<DescriptorEntry>()) as u64;

        self.translator.translate(addr)
    }

    pub fn address_of_tx_descriptor(&self, idx: usize) -> u32 {
        assert!(idx < self.transmit_descriptors.len());

        let base_addr = &*self.transmit_descriptors as *const TransmitDescriptors as u64;
        let addr = base_addr + (idx * size_of::<DescriptorEntry>()) as u64;

        self.translator.translate(addr)
    }

    pub fn address_of_rx_buffer(&self, idx: usize) -> u32 {
        assert!(idx < self.receive_buffers.len());

        let base_addr = &*self.receive_buffers as *const ReceiveBuffers as u64;
        let addr = base_addr + (idx * size_of::<PacketBuffer>()) as u64;

        self.translator.translate(addr)
    }

    pub fn address_of_tx_buffer(&self, idx: usize) -> u32 {
        assert!(idx < self.transmit_buffers.len());

        let base_addr = &*self.transmit_buffers as *const TransmitBuffers as u64;
        let addr = base_addr + (idx * size_of::<PacketBuffer>()) as u64;

        self.translator.translate(addr)
    }

    pub fn address_of_init_block(&self) -> u32 {
        let addr = &*self.init_block as *const InitBlock as u64;

        self.translator.translate(addr)
    }

    pub fn initialize(&mut self, mac: [u8; 6]) {
        // Initialize desciptors
        for idx in 0..NUM_RECEIVE_BUFFERS {
            let mut desc = self.receive_descriptors.entries[idx];
            let buf_addr = self.address_of_rx_buffer(idx);

            desc.buffer_address = buf_addr;
            desc.set_size(BUFFER_SIZE);
            desc.set_card_ownership();
        }

        for idx in 0..NUM_TRANSMIT_BUFFERS {
            let mut desc = self.transmit_descriptors.entries[idx];
            let buf_addr = self.address_of_tx_buffer(idx);

            desc.buffer_address = buf_addr;
            desc.set_size(BUFFER_SIZE);
        }

        // Initialize init struct
        self.init_block.mode = 0x0;
        self.init_block.set_rlen(NUM_RECEIVE_BUFFERS as u16);
        self.init_block.set_tlen(NUM_TRANSMIT_BUFFERS as u16);
        self.init_block.mac0 = mac[0];
        self.init_block.mac1 = mac[1];
        self.init_block.mac2 = mac[2];
        self.init_block.mac3 = mac[3];
        self.init_block.mac4 = mac[4];
        self.init_block.mac5 = mac[5];
        self.init_block.rx_desc_phys_addr = self.address_of_rx_descriptor(0);
        self.init_block.tx_desc_phys_addr = self.address_of_tx_descriptor(0);
    }
}
