const CLASS_MAP: [(u8, &'static str); 4] = [
    (0x01, "Mass Storage Controller"),
    (0x02, "Network Controller"),
    (0x03, "Display Controller"),
    (0x06, "Bridge"),
];

pub fn get_class_name(class_id: u8) -> Option<&'static str> {
    for (id, name) in CLASS_MAP.iter() {
        if class_id == *id {
            return Some(*name);
        }
    }

    None
}
