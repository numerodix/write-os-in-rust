use alloc::{format, string::String};

#[macro_export]
macro_rules! print_all {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print(format_args!($($arg)*));
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println_all {
    () => ($crate::print_all!("\n"));
    ($fmt:expr) => ($crate::print_all!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_all!(
        concat!($fmt, "\n"), $($arg)*));
}

pub fn format_mac_address(mac: [u8; 6]) -> String {
    format!(
        "{:>02X}:{:>02X}:{:>02X}:{:>02X}:{:>02X}:{:>02X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}
