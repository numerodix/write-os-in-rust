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
pub struct ReceiveBuffers {
    buffers: [PacketBuffer; 32],
}

impl ReceiveBuffers {
    fn new() -> Self {
        Self {
            buffers: [PacketBuffer { buffer: [0; 1520] }; 32],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct TransmitBuffers {
    buffers: [PacketBuffer; 8],
}

impl TransmitBuffers {
    fn new() -> Self {
        Self {
            buffers: [PacketBuffer { buffer: [0; 1520] }; 8],
        }
    }
}

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
}
