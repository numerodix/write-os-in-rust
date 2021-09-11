const AMD_ATI: &'static str = "Advanced Micro Devices, Inc. [AMD/ATI]";
const AMD: &'static str = "Advanced Micro Devices, Inc. [AMD]";
const IBM: &'static str = "IBM";
const INTEL: &'static str = "Intel Corporation";
const REALTEK: &'static str = "Realtek Semiconductor Co., Ltd.";
const UBIQUITY: &'static str = "Ubiquiti Networks, Inc.";

// https://pci-ids.ucw.cz/read/PC?restrict=
const VENDOR_MAP: [(u16, &'static str); 6] = [
    (0x0777, UBIQUITY),
    (0x1002, AMD_ATI),
    (0x1014, IBM),
    (0x1022, AMD),
    (0x10EC, REALTEK),
    (0x8086, INTEL),
];

pub fn get_vendor_name(class_code: u16) -> Option<&'static str> {
    for (id, name) in VENDOR_MAP.iter() {
        if class_code == *id {
            return Some(*name);
        }
    }

    None
}
