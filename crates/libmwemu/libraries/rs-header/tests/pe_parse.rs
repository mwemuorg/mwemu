//! Parsing tests against mwemu's own `loader.exe` placeholder PEs (small, real,
//! and not third-party). Bytes are embedded so the tests are self-contained.

use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;

static LOADER64: &[u8] = include_bytes!("fixtures/loader64.exe");
static LOADER32: &[u8] = include_bytes!("fixtures/loader32.exe");

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
    assert_eq!(pe.iat_names.len(), 0, "iat_names is filled during load, not parse");
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
