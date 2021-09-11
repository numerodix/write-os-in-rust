const SUBCLASS_MAP: [(u16, &'static str); 6] = [
    // Mass Storage Controller
    (0x0101, "IDE Controller"),
    // Network Controller
    (0x0200, "Ethernet Controller"),
    // Display Controller
    (0x0300, "VGA Compatible Controller"),
    // Bridge
    (0x0600, "Host Bridge"),
    (0x0601, "ISA Bridge"),
    (0x0680, "Other"),
];

pub fn get_subclass_name(class_id: u8, subclass_id: u8) -> Option<&'static str> {
    let key: u16 = (class_id as u16) << 8 | (subclass_id as u16);

    for (id, name) in SUBCLASS_MAP.iter() {
        if key == *id {
            return Some(*name);
        }
    }

    None
}
