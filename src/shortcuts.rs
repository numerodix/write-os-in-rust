use crate::{println, serial_println};

pub fn println_both(msg: &str) {
    println!("{}", msg);
    serial_println!("{}", msg);
}
