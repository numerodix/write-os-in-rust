use core::mem::size_of;

use alloc::boxed::Box;

use super::support::AddrTranslator;

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
pub struct PacketBuffer {
    buffer: [u8; 1520],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct PacketRingBuffer<const S: usize> {
    buffers: [PacketBuffer; S],
}

impl<const S: usize> PacketRingBuffer<S> {
    fn new() -> Self {
        Self {
            buffers: [PacketBuffer { buffer: [0; 1520] }; S],
        }
    }

    pub fn len(&self) -> usize {
        S
    }
}

type ReceiveBuffers = PacketRingBuffer<32>;
type TransmitBuffers = PacketRingBuffer<8>;

pub struct BufferManager {
    translator: AddrTranslator,

    receive_buffers: Box<ReceiveBuffers>,
    transmit_buffers: Box<TransmitBuffers>,
}

impl BufferManager {
    pub fn new(physical_memory_offset: u64) -> Self {
        Self {
            translator: AddrTranslator::new(physical_memory_offset),
            receive_buffers: Box::new(ReceiveBuffers::new()),
            transmit_buffers: Box::new(TransmitBuffers::new()),
        }
    }

    pub fn address_of_rx_buffer(&self, idx: usize) -> u32 {
        assert!(idx < self.receive_buffers.len());

        let base_addr = &*self.receive_buffers as *const ReceiveBuffers as u64;
        let addr = base_addr + (idx * size_of::<PacketBuffer>()) as u64;

        self.translator.translate(addr)
    }

    pub fn address_of_tx_buffer(&self, idx: usize) -> u32 {
        assert!(idx < self.receive_buffers.len());

        let base_addr = &*self.receive_buffers as *const ReceiveBuffers as u64;
        let addr = base_addr + (idx * size_of::<PacketBuffer>()) as u64;

        self.translator.translate(addr)
    }
}
