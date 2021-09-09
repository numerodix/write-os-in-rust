#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use blog_os::allocator::init_allocation_system;
use blog_os::println;
use blog_os::task::keyboard;
use blog_os::task::{executor::Executor, Task};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDevice {
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
}

pub fn read_dword(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    use x86_64::instructions::port::Port;

    let offset = offset & 0xFC;
    let add: u32 = 0x80000000
        | (u32::from(bus) << 16)
        | (u32::from(device) << 11)
        | (u32::from(function) << 8)
        | u32::from(offset);

    let mut port_out = Port::new(0xCF8);
    let mut port_in = Port::new(0xCFC);
    unsafe { port_out.write(add) };
    let reply: u32 = unsafe { port_in.read() };

    reply >> ((offset & 2) * 8)
}

pub fn read_pci_device(bus: u8, device: u8) -> PciDevice {
    let signature = read_dword(bus, device, 0, 0);
    let class_dword = read_dword(bus, device, 0, 8);

    let vendor: u16 = (signature & 0xffff) as u16;
    let device: u16 = (signature >> 16) as u16;
    let class = (class_dword >> 24) as u8;

    PciDevice {
        vendor,
        device,
        class,
    }
}

#[no_mangle]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    blog_os::init();
    init_allocation_system(boot_info);
    use blog_os::serial_println;

    #[cfg(test)]
    test_main();

    for bus_id in 0..1 {
        for dev_id in 0..4 {
            // let mut offset = 0;
            // let val = read_dword(bus_id, dev_id, 0, offset);
            // serial_println!("bus: {}  dev: {}  offset: {}  got: 0x{:x}", bus_id, dev_id, offset, val);

            // offset = 4;
            // let val = read_dword(bus_id, dev_id, 0, offset);
            // serial_println!("bus: {}  dev: {}  offset: {}  got: 0x{:x}", bus_id, dev_id, offset, val);

            // let offset = 8;
            // let val = read_dword(bus_id, dev_id, 0, offset);
            // serial_println!("bus: {}  dev: {}  offset: {}  got: 0x{:x}", bus_id, dev_id, offset, val);

            // serial_println!();

            let dev = read_pci_device(bus_id, dev_id);
            serial_println!(
                "bus: {}  dev: {}  ::  vendor: 0x{:x}  device: 0x{:x}  class: 0x{:x}",
                bus_id,
                dev_id,
                dev.vendor,
                dev.device,
                dev.class,
            );
        }
    }

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
