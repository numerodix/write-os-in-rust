use core::mem;

#[repr(packed)]
pub struct DescriptorEntry {
    buffer_address: u32, // 4 bytes
    count: u16,          // 2 bytes
    unused1: u8,         // 1 byte
    ownership: u8,       // 1 byte
    unused2: u32,        // 4 bytes
    unused3: u32,        // 4 bytes
}

#[repr(packed)]
pub struct PacketBuffer {
    buffer: [u8; 1520],
}

#[repr(packed)]
pub struct ReceiveBuffers {
    buffers: [PacketBuffer; 32],
}

impl ReceiveBuffers {
    pub fn size() -> usize {
        mem::size_of::<Self>()
    }
}

#[repr(packed)]
pub struct TransmitBuffers {
    buffers: [PacketBuffer; 8],
}
