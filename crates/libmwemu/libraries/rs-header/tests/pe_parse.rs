//! Parsing tests against mwemu's own `loader.exe` placeholder PEs (small, real,
//! and not third-party). Bytes are embedded so the tests are self-contained.

// clippy v1 burn-down backlog (see V1-ROADMAP.md P2 #9)
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::unusual_byte_groupings)]

pub mod fixtures;

use fixtures::{LOADER32, LOADER64};
use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;

#[test]
fn parse_pe64() {
    let pe = PE64::parse("loader64.exe", LOADER64);

    assert_eq!(pe.dos.e_magic, 0x5a4d, "MZ signature");
    assert!(pe.num_of_sections() > 0, "should have sections");

    // borrow-based accessors work against caller-owned bytes
    let headers = pe.headers(LOADER64);
    assert!(!headers.is_empty());
    assert_eq!(&headers[0..2], b"MZ");

    // every section maps back into the file image
    for i in 0..pe.num_of_sections() {
        let _ = pe.get_section_ptr(LOADER64, i); // must not panic
        assert!(!pe.get_section(i).get_name().is_empty());
    }

    // the struct does NOT own the file bytes anymore
    assert_eq!(
        pe.iat_names.len(),
        0,
        "iat_names is filled during load, not parse"
    );
}

#[test]
fn parse_pe32() {
    let pe = PE32::parse("loader32.exe", LOADER32);

    assert_eq!(pe.dos.e_magic, 0x5a4d, "MZ signature");
    assert!(pe.num_of_sections() > 0, "should have sections");

    let headers = pe.headers(LOADER32);
    assert_eq!(&headers[0..2], b"MZ");
}

#[test]
fn pe64_rejects_pe32_and_viceversa_by_magic() {
    // both loaders are valid PEs; just assert the optional-header magic differs
    let pe64 = PE64::parse("x", LOADER64);
    let pe32 = PE32::parse("x", LOADER32);
    assert_eq!(pe64.opt.magic, 0x20b, "PE32+ optional header magic");
    assert_eq!(pe32.opt.magic, 0x10b, "PE32 optional header magic");
}

#[test]
fn pe32_section_raw_data_respects_file_bounds() {
    let mut pe = PE32::parse("loader32.exe", LOADER32);
    let raw = b"section!";
    let cases: &[(u32, u32, &[u8])] = &[
        (2, 4, b"ctio"),           // Section ends before EOF.
        (2, 6, b"ction!"),         // Section ends exactly at EOF.
        (2, 7, b"ction!"),         // Declared section extends past EOF.
        (7, 1, b"!"),              // Single byte at EOF.
        (7, 2, b"!"),              // Single available byte after truncation.
        (2, u32::MAX, b"ction!"),  // Large declared size is clamped safely.
        (2, 0, b""),               // Empty section within the file.
        (8, 1, b""),               // Raw offset at EOF.
        (9, 1, b""),               // Raw offset past EOF.
        (u32::MAX, u32::MAX, b""), // Large offset must not underflow.
    ];
    for &(offset, size, expected) in cases {
        pe.sect_hdr[0].pointer_to_raw_data = offset;
        pe.sect_hdr[0].size_of_raw_data = size;
        assert_eq!(
            pe.get_section_ptr(raw, 0),
            expected,
            "raw offset {offset}, raw size {size}"
        );
    }
}

#[test]
fn pe32_section_raw_data_is_empty_without_backing_bytes() {
    let mut pe = PE32::parse("loader32.exe", LOADER32);
    for offset in [0, 1, u32::MAX] {
        pe.sect_hdr[0].pointer_to_raw_data = offset;
        pe.sect_hdr[0].size_of_raw_data = u32::MAX;
        assert!(pe.get_section_ptr(&[], 0).is_empty(), "raw offset {offset}");
    }
}

#[test]
fn pe_headers_respect_file_bounds() {
    let mut pe32 = PE32::parse("loader32.exe", LOADER32);
    let mut pe64 = PE64::parse("loader64.exe", LOADER64);
    let cases: &[(&[u8], u32, &[u8])] = &[
        (b"headers!", 0, b""),
        (b"headers!", 3, b"hea"),
        (b"headers!", 8, b"headers!"), // Exactly at EOF.
        (b"headers!", 9, b"headers!"), // Declared size extends past EOF.
        (b"headers!", u32::MAX, b"headers!"),
        (b"x", 1, b"x"),
        (b"x", 2, b"x"),
        (b"", 0, b""),
        (b"", u32::MAX, b""),
    ];
    for &(raw, declared, expected) in cases {
        pe32.opt.size_of_headers = declared;
        pe64.opt.size_of_headers = declared;
        assert_eq!(pe32.headers(raw), expected, "PE32 size {declared}");
        assert_eq!(pe64.headers(raw), expected, "PE64 size {declared}");
    }
}
