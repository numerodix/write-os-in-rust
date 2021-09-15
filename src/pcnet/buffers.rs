use core::mem::size_of;

use alloc::boxed::Box;

use super::support::AddrTranslator;

const NUM_RECEIVE_BUFFERS: usize = 32;
const NUM_TRANSMIT_BUFFERS: usize = 8;
const BUFFER_SIZE: usize = 1520;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct DescriptorEntry {
    buffer_address: u32, // 4 bytes
    count: u16,          // 2 bytes
    unused1: u8,         // 1 byte
    ownership: u8,       // 1 byte
    unused2: u32,        // 4 bytes
    unused3: u32,        // 4 bytes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct DescriptorRingBuffer<const S: usize> {
    entries: [DescriptorEntry; S],
}

impl<const S: usize> DescriptorRingBuffer<S> {
    fn new() -> Self {
        Self {
            entries: [DescriptorEntry {
                buffer_address: 0,
                count: 0,
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
    buffer: [u8; BUFFER_SIZE],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct PacketRingBuffer<const S: usize> {
    buffers: [PacketBuffer; S],
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
pub struct InitStruct {
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

impl InitStruct {
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
}

pub struct BufferManager {
    translator: AddrTranslator,

    receive_buffers: Box<ReceiveBuffers>,
    transmit_buffers: Box<TransmitBuffers>,

    receive_descriptors: Box<ReceiveDescriptors>,
    transmit_descriptors: Box<TransmitDescriptors>,

    init_struct: Box<InitStruct>,
}

impl BufferManager {
    pub fn new(physical_memory_offset: u64) -> Self {
        Self {
            translator: AddrTranslator::new(physical_memory_offset),

            receive_buffers: Box::new(ReceiveBuffers::new()),
            transmit_buffers: Box::new(TransmitBuffers::new()),

            receive_descriptors: Box::new(ReceiveDescriptors::new()),
            transmit_descriptors: Box::new(TransmitDescriptors::new()),

            init_struct: Box::new(InitStruct::new()),
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

    pub fn address_of_init_struct(&self) -> u32 {
        let addr = &*self.init_struct as *const InitStruct as u64;

        self.translator.translate(addr)
    }

    pub fn initialize(&mut self) {
        for idx in 0..NUM_RECEIVE_BUFFERS {
            let mut desc = self.receive_descriptors.entries[idx];
            let addr = self.address_of_rx_buffer(idx);
            desc.buffer_address = addr;
        }
    }
}
