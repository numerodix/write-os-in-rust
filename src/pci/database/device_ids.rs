// https://www.pcilookup.com/?ven=10ec&dev=8139&action=submit
const DEVICE_MAP: [(u16, u16, &'static str); 7] = [
    (
        0x10EC,
        0x8139,
        "RTL-8100/8101L/8139 PCI Fast Ethernet Adapter",
    ),
    (0x1022, 0x2000, "79c970 [PCnet32 LANCE]"),
    (0x8086, 0x1237, "440FX - 82441FX PMC [Natoma]"),
    (0x8086, 0x7000, "82371SB PIIX3 ISA [Natoma/Triton II]"),
    (0x8086, 0x7010, "82371SB PIIX3 IDE [Natoma/Triton II]"),
    (0x8086, 0x7113, "82371AB/EB/MB PIIX4 ACPI"),
    (0x8086, 0x100E, "82540EM Gigabit Ethernet Controller"),
];

pub fn get_device_name(vendor_id: u16, device_id: u16) -> Option<&'static str> {
    for (vendor, device, name) in DEVICE_MAP.iter() {
        if vendor_id == *vendor && device_id == *device {
            return Some(*name);
        }
    }

    None
}
