use x86_64::instructions::port::Port;

pub const CONFIG_ADDRESS: u16 = 0xCF8;
pub const CONFIG_DATA: u16 = 0xCFC;

fn get_address(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let offset = offset & 0xFC;

    0x80000000
        | (u32::from(bus) << 16)
        | (u32::from(device) << 11)
        | (u32::from(function) << 8)
        | u32::from(offset)
}

pub fn config_read(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let mut address_port = Port::new(CONFIG_ADDRESS);
    let mut data_port = Port::new(CONFIG_DATA);

    let address = get_address(bus, device, function, offset);

    unsafe { address_port.write(address) };
    let value: u32 = unsafe { data_port.read() };

    value
}

pub fn config_write(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    let mut address_port = Port::new(CONFIG_ADDRESS);
    let mut data_port = Port::new(CONFIG_DATA);

    let address = get_address(bus, device, function, offset);

    unsafe { address_port.write(address) };
    unsafe { data_port.write(value) };
}
