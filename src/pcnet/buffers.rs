use alloc::boxed::Box;

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

    pub fn alloc() -> Box<Self> {
        let bufs = Box::new(Self::new());
        // let ptr = &*bufs as *const ReceiveBuffers;
        // assert physical address fits in u32?
        bufs
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

    pub fn alloc() -> Box<Self> {
        let bufs = Box::new(Self::new());
        // let ptr = &*bufs as *const ReceiveBuffers;
        // assert physical address fits in u32?
        bufs
    }
}
