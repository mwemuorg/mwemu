mod fixtures;

use fixtures::{LOADER32, LOADER64};
use rs_header::pe::export_index::{ExportTarget, build_image_export_index};
use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;
use rs_header::pe::shared::IMAGE_DIRECTORY_ENTRY_RESOURCE;

const HEADERS: usize = 0x400;
const RAW: usize = 0x800;
const SECTION_RVA: u32 = 0x1000;
const DLL_NAME: &[u8] = b"kernel32.dll\0";

fn write_u32(raw: &mut [u8], offset: usize, value: u32) {
    raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

struct Fixture {
    raw: Vec<u8>,
    opt: usize,
    sections: usize,
    directories: usize,
}

fn fixture(is_64: bool) -> Fixture {
    let original = if is_64 { LOADER64 } else { LOADER32 };
    let mut raw = vec![0; 0x1800];
    raw[..HEADERS].copy_from_slice(&original[..HEADERS]);
    let nt = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
    let opt = nt + 24;
    let sections = opt + u16::from_le_bytes(raw[nt + 20..nt + 22].try_into().unwrap()) as usize;
    let directories = opt + if is_64 { 112 } else { 96 };
    raw[nt + 6..nt + 8].copy_from_slice(&1u16.to_le_bytes());
    write_u32(&mut raw, opt + 32, 0x1000);
    write_u32(&mut raw, opt + 56, 0x2000);
    write_u32(&mut raw, opt + 60, HEADERS as u32);
    raw[directories..directories + 16 * 8].fill(0);
    raw[sections..HEADERS].fill(0);
    raw[sections..sections + 8].copy_from_slice(b".data\0\0\0");
    write_u32(&mut raw, sections + 8, 0x1000);
    write_u32(&mut raw, sections + 12, SECTION_RVA);
    write_u32(&mut raw, sections + 16, 0x200);
    write_u32(&mut raw, sections + 20, RAW as u32);
    Fixture {
        raw,
        opt,
        sections,
        directories,
    }
}

fn imports(is_64: bool, raw: &[u8]) -> Vec<String> {
    if is_64 {
        PE64::parse("layout.exe", raw)
            .image_import_descriptor
            .into_iter()
            .map(|d| d.name)
            .collect()
    } else {
        PE32::parse("layout.exe", raw)
            .image_import_descriptor
            .into_iter()
            .map(|d| d.name)
            .collect()
    }
}

#[test]
fn import_names_can_live_in_headers() {
    for is_64 in [false, true] {
        let mut f = fixture(is_64);
        let name = HEADERS - DLL_NAME.len();
        write_u32(&mut f.raw, f.directories + 8, SECTION_RVA);
        write_u32(&mut f.raw, RAW + 12, name as u32);
        f.raw[name..HEADERS].copy_from_slice(DLL_NAME);
        assert_eq!(imports(is_64, &f.raw), ["kernel32.dll"], "is_64={is_64}");
    }
}

#[test]
fn metadata_uses_sector_aligned_raw_offsets() {
    for is_64 in [false, true] {
        let mut f = fixture(is_64);
        write_u32(&mut f.raw, f.sections + 20, RAW as u32 + 1);
        write_u32(&mut f.raw, f.directories + 8, SECTION_RVA);
        write_u32(&mut f.raw, RAW + 12, SECTION_RVA + 0x80);
        f.raw[RAW + 0x80..RAW + 0x80 + DLL_NAME.len()].copy_from_slice(DLL_NAME);
        assert_eq!(imports(is_64, &f.raw), ["kernel32.dll"], "is_64={is_64}");
    }
}

#[test]
fn section_access_rejects_invalid_indices() {
    let f32 = fixture(false);
    let f64 = fixture(true);
    let pe32 = PE32::parse("layout.exe", &f32.raw);
    let pe64 = PE64::parse("layout.exe", &f64.raw);
    for index in [1, 2, usize::MAX] {
        assert!(pe32.get_section_ptr(&f32.raw, index).is_empty());
        assert!(pe64.get_section_ptr(&f64.raw, index).is_empty());
    }
}

#[test]
fn named_section_access_uses_available_raw_bytes() {
    for is_64 in [false, true] {
        let mut f = fixture(is_64);
        write_u32(&mut f.raw, f.sections + 8, u32::MAX);
        f.raw.truncate(RAW + 17);
        let data = if is_64 {
            PE64::parse("layout.exe", &f.raw).get_section_ptr_by_name(&f.raw, ".data")
        } else {
            PE32::parse("layout.exe", &f.raw).get_section_ptr_by_name(&f.raw, ".data")
        };
        assert_eq!(data, Some(&f.raw[RAW..]), "is_64={is_64}");
    }
}

#[test]
fn tls_directory_and_callbacks_follow_image_layout() {
    for is_64 in [false, true] {
        for in_headers in [false, true] {
            let mut f = fixture(is_64);
            let (directory_rva, directory_off, callbacks_rva, callbacks_off) = if in_headers {
                (0x300, 0x300, 0x350, 0x350)
            } else {
                (SECTION_RVA, RAW, SECTION_RVA + 0x80, RAW + 0x80)
            };
            write_u32(&mut f.raw, f.sections + 20, RAW as u32 + 1);
            write_u32(&mut f.raw, f.directories + 9 * 8, directory_rva);
            let callbacks = if is_64 {
                let base = PE64::parse("tls.exe", &f.raw).opt.image_base;
                f.raw[directory_off + 24..directory_off + 32]
                    .copy_from_slice(&(base + callbacks_rva as u64).to_le_bytes());
                f.raw[callbacks_off..callbacks_off + 8]
                    .copy_from_slice(&(base + 0x1234).to_le_bytes());
                let pe = PE64::parse("tls.exe", &f.raw);
                assert_eq!(
                    pe.get_tls_directory(&f.raw).unwrap().tls_callbacks,
                    base + callbacks_rva as u64
                );
                (pe.get_tls_callbacks(&f.raw, 0), base + 0x1234)
            } else {
                let base = PE32::parse("tls.exe", &f.raw).opt.image_base;
                write_u32(&mut f.raw, directory_off + 12, base + callbacks_rva);
                write_u32(&mut f.raw, callbacks_off, base + 0x1234);
                let pe = PE32::parse("tls.exe", &f.raw);
                assert_eq!(
                    pe.get_tls_directory(&f.raw).unwrap().tls_callbacks,
                    base + callbacks_rva
                );
                (pe.get_tls_callbacks(&f.raw, 0), (base + 0x1234) as u64)
            };
            assert_eq!(
                callbacks.0,
                [callbacks.1],
                "is_64={is_64}, headers={in_headers}"
            );
        }
    }
}

#[test]
fn invalid_metadata_rvas_do_not_read_dos_header() {
    for is_64 in [false, true] {
        for rva in [0x800, 0x1800, u32::MAX] {
            let mut f = fixture(is_64);
            write_u32(&mut f.raw, f.directories + 8, SECTION_RVA);
            write_u32(&mut f.raw, RAW + 12, rva);
            write_u32(&mut f.raw, f.directories + 9 * 8, rva);
            assert!(imports(is_64, &f.raw).is_empty());
            if is_64 {
                assert!(
                    PE64::parse("invalid.exe", &f.raw)
                        .get_tls_directory(&f.raw)
                        .is_none()
                );
            } else {
                assert!(
                    PE32::parse("invalid.exe", &f.raw)
                        .get_tls_directory(&f.raw)
                        .is_none()
                );
            }
        }
    }
}

#[test]
fn low_alignment_metadata_can_live_between_sections() {
    for is_64 in [false, true] {
        let mut f = fixture(is_64);
        write_u32(&mut f.raw, f.opt + 32, 0x200);
        write_u32(&mut f.raw, f.sections + 12, RAW as u32);
        write_u32(&mut f.raw, f.sections + 8, 0x20);
        write_u32(&mut f.raw, f.sections + 16, 0x20);
        write_u32(&mut f.raw, f.directories + 8, 0x600);
        write_u32(&mut f.raw, 0x600 + 12, 0x700);
        f.raw[0x700..0x700 + DLL_NAME.len()].copy_from_slice(DLL_NAME);
        assert_eq!(imports(is_64, &f.raw), ["kernel32.dll"]);
    }
}

#[test]
fn export_names_and_forwarders_follow_image_layout() {
    for is_64 in [false, true] {
        for in_headers in [false, true] {
            let mut f = fixture(is_64);
            write_u32(&mut f.raw, f.sections + 20, RAW as u32 + 1);
            let (rva, offset) = if in_headers {
                (0x300, 0x300)
            } else {
                (SECTION_RVA, RAW)
            };
            for (field, value) in [
                (16, 7),
                (20, 2),
                (24, 1),
                (28, rva + 0x40),
                (32, rva + 0x50),
                (36, rva + 0x60),
            ] {
                write_u32(&mut f.raw, offset + field, value);
            }
            write_u32(&mut f.raw, offset + 0x40, 0x1500);
            write_u32(&mut f.raw, offset + 0x44, rva + 0x70);
            write_u32(&mut f.raw, offset + 0x50, rva + 0x90);
            f.raw[offset + 0x70..offset + 0x70 + 15].copy_from_slice(b"KERNEL32.Sleep\0");
            f.raw[offset + 0x90..offset + 0x96].copy_from_slice(b"Probe\0");
            let index = if is_64 {
                let pe = PE64::parse("exports.dll", &f.raw);
                build_image_export_index(&f.raw, pe.image_layout(), rva, 0x100)
            } else {
                let pe = PE32::parse("exports.dll", &f.raw);
                build_image_export_index(&f.raw, pe.image_layout(), rva, 0x100)
            }
            .unwrap();
            assert_eq!(index.export_base, 7);
            assert_eq!(
                index.ordinal_targets,
                [
                    Some(ExportTarget::Direct { rva: 0x1500 }),
                    Some(ExportTarget::Forwarder {
                        value: "KERNEL32.Sleep".into()
                    })
                ]
            );
            assert_eq!(index.named_exports[0].name, "Probe");
        }
    }
}

#[test]
fn ordinal_only_exports_do_not_require_name_tables() {
    let mut f = fixture(true);
    write_u32(&mut f.raw, RAW + 16, 3);
    write_u32(&mut f.raw, RAW + 20, 1);
    write_u32(&mut f.raw, RAW + 28, SECTION_RVA + 0x40);
    write_u32(&mut f.raw, RAW + 0x40, 0x1500);
    let pe = PE64::parse("ordinal.dll", &f.raw);
    let index = build_image_export_index(&f.raw, pe.image_layout(), SECTION_RVA, 0x100).unwrap();
    assert_eq!(
        index.ordinal_targets,
        [Some(ExportTarget::Direct { rva: 0x1500 })]
    );
    assert!(index.named_exports.is_empty());
}

const RESOURCE_FLAG: u32 = 1 << 31;
const RESOURCE_TYPE: u32 = 10;
const RESOURCE_NAME: u32 = 300;
const RESOURCE_LANGUAGE: u32 = 1033;

fn resource_tree(raw: &mut [u8], base: usize, payload: u32, named: bool) {
    for (offset, id, child) in [
        (0, RESOURCE_TYPE, 0x20),
        (0x20, RESOURCE_NAME, 0x40),
        (0x40, RESOURCE_LANGUAGE, 0x60),
    ] {
        let named_entry = named && offset < 0x40;
        let count_offset = if named_entry { 12 } else { 14 };
        raw[base + offset + count_offset..base + offset + count_offset + 2]
            .copy_from_slice(&1u16.to_le_bytes());
        let name = if named_entry {
            RESOURCE_FLAG | (0x80 + offset) as u32
        } else {
            id
        };
        write_u32(raw, base + offset + 16, name);
        write_u32(
            raw,
            base + offset + 20,
            child | if offset < 0x40 { RESOURCE_FLAG } else { 0 },
        );
    }
    write_u32(raw, base + 0x60, payload);
    write_u32(raw, base + 0x64, 4);
    for (offset, name) in [(0x80, "CUSTOMTYPE"), (0xa0, "CUSTOMNAME")] {
        let units: Vec<u16> = name.encode_utf16().collect();
        raw[base + offset..base + offset + 2].copy_from_slice(&(units.len() as u16).to_le_bytes());
        for (index, unit) in units.into_iter().enumerate() {
            raw[base + offset + 2 + index * 2..base + offset + 4 + index * 2]
                .copy_from_slice(&unit.to_le_bytes());
        }
    }
}

fn resource(is64: bool, raw: &[u8], named: bool) -> Option<(u64, usize)> {
    let (type_id, name_id, type_name, name) = if named {
        (None, None, Some("CUSTOMTYPE"), Some("CUSTOMNAME"))
    } else {
        (Some(RESOURCE_TYPE), Some(RESOURCE_NAME), None, None)
    };
    if is64 {
        PE64::parse("resource.exe", raw).get_resource(raw, type_id, name_id, type_name, name)
    } else {
        PE32::parse("resource.exe", raw).get_resource(raw, type_id, name_id, type_name, name)
    }
}

#[test]
fn resource_rvas_follow_image_layout_for_ids_and_names() {
    for is64 in [false, true] {
        for named in [false, true] {
            for in_headers in [false, true] {
                let mut f = fixture(is64);
                let (rva, offset) = if in_headers {
                    (0x300, 0x300)
                } else {
                    (SECTION_RVA, RAW)
                };
                write_u32(
                    &mut f.raw,
                    f.directories + IMAGE_DIRECTORY_ENTRY_RESOURCE * 8,
                    rva,
                );
                write_u32(
                    &mut f.raw,
                    f.directories + IMAGE_DIRECTORY_ENTRY_RESOURCE * 8 + 4,
                    0xc0,
                );
                // The directory has no section-name dependency; the payload is separate.
                resource_tree(&mut f.raw, offset, SECTION_RVA + 0x100, named);
                f.raw[RAW + 0x100..RAW + 0x104].copy_from_slice(b"DATA");
                assert_eq!(
                    resource(is64, &f.raw, named),
                    Some(((SECTION_RVA + 0x100) as u64, 4)),
                    "x64={is64}, named={named}, headers={in_headers}"
                );
            }
        }
    }
}

#[test]
fn malformed_resource_trees_are_bounded_and_do_not_substitute_file_offsets() {
    for is64 in [false, true] {
        let mut f = fixture(is64);
        write_u32(
            &mut f.raw,
            f.directories + IMAGE_DIRECTORY_ENTRY_RESOURCE * 8,
            SECTION_RVA,
        );
        write_u32(
            &mut f.raw,
            f.directories + IMAGE_DIRECTORY_ENTRY_RESOURCE * 8 + 4,
            0xc0,
        );
        resource_tree(&mut f.raw, RAW, SECTION_RVA + 0x100, false);
        for (label, offset, value) in [
            ("child outside directory", RAW + 20, RESOURCE_FLAG | 0xc0),
            ("directory cycle", RAW + 20, RESOURCE_FLAG),
            ("early data leaf", RAW + 20, 0x60),
            (
                "directory instead of data",
                RAW + 0x54,
                RESOURCE_FLAG | 0x60,
            ),
            ("truncated data entry", RAW + 0x54, 0xb8),
            ("payload outside image", RAW + 0x60, u32::MAX),
            ("overflowing payload length", RAW + 0x64, u32::MAX),
            (
                "truncated declared directory",
                f.directories + IMAGE_DIRECTORY_ENTRY_RESOURCE * 8 + 4,
                15,
            ),
            ("malformed name offset", RAW + 16, RESOURCE_FLAG | 0xbf),
        ] {
            let mut raw = f.raw.clone();
            write_u32(&mut raw, offset, value);
            assert!(resource(is64, &raw, false).is_none(), "{label}, x64={is64}");
        }
        for len in [0, RAW, RAW + 16, RAW + 0x60] {
            assert!(resource(is64, &f.raw[..len], false).is_none());
        }
    }
}
