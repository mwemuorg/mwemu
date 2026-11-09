use rs_header::pe::image::{aligned_size, image_size, section_data, section_size};
use rs_header::pe::pe32::PE32;

pub mod fixtures;
use fixtures::LOADER32;

#[test]
fn mapped_section_data_is_bounded_by_file_and_virtual_extent() {
    let mut pe = PE32::parse("loader32.exe", LOADER32);
    let mut section = pe.sect_hdr.remove(0);
    let raw = vec![0x5a; 0x901];
    section.virtual_size = 0x20;
    section.pointer_to_raw_data = 0x801;
    section.size_of_raw_data = u32::MAX;
    assert_eq!(section_data(&raw, &section, 0x1000), &raw[0x800..]);
    section.pointer_to_raw_data = u32::MAX;
    assert!(section_data(&raw, &section, 0x1000).is_empty());
    section.pointer_to_raw_data = 0x801;
    section.size_of_raw_data = 0;
    assert!(section_data(&raw, &section, 0x1000).is_empty());
    section.virtual_size = 0;
    section.size_of_raw_data = 0x201;
    assert_eq!(section_size(&section, 0x1000), 0x1000);
}

#[test]
fn image_extent_includes_headers_gaps_and_rounded_tails() {
    let mut pe = PE32::parse("loader32.exe", LOADER32);
    let mut section = pe.sect_hdr.remove(0);
    section.virtual_address = 0x10000;
    section.virtual_size = 1;
    assert_eq!(image_size(0x400, 0x2000, &[section], 0x1000), 0x11000);
    assert_eq!(image_size(0x1001, 0, &[], 0x1000), 0x2000);
    assert_eq!(image_size(0x400, 0x20000, &[], 0x1000), 0x20000);
    assert_eq!(aligned_size(u32::MAX, 0x1000), 0x1_0000_0000);
}
