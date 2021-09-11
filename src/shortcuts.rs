use crate::{print, println, serial_print, serial_println};

pub fn print_both(msg: &str) {
    print!("{}", msg);
    serial_print!("{}", msg);
}

pub fn println_both(msg: &str) {
    println!("{}", msg);
    serial_println!("{}", msg);
}
