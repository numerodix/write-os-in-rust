use alloc::{format, string::String};

use crate::{print, println, serial_print, serial_println};

pub fn print_both(msg: &str) {
    print!("{}", msg);
    serial_print!("{}", msg);
}

pub fn println_both(msg: &str) {
    println!("{}", msg);
    serial_println!("{}", msg);
}

pub fn format_mac_address(mac: [u8; 6]) -> String {
    format!(
        "{:>02X}:{:>02X}:{:>02X}:{:>02X}:{:>02X}:{:>02X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}
