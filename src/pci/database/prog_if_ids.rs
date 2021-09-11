const PROG_IF_MAP: [(u32, &'static str); 2] = [
    // Mass Storage Controller
    (
        0x010180,
        "ISA Compatibility mode-only controller, supports bus mastering",
    ),
    // Display Controller
    (0x030000, "VGA Controller"),
];

pub fn get_prog_if_name(class_id: u8, subclass_id: u8, prog_if_id: u8) -> Option<&'static str> {
    let key: u32 = (class_id as u32) << 16 | (subclass_id as u32) << 8 | (prog_if_id as u32);

    for (id, name) in PROG_IF_MAP.iter() {
        if key == *id {
            return Some(*name);
        }
    }

    None
}
