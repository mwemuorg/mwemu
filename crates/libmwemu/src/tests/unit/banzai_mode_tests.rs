use std::io::Write;

use tempfile::NamedTempFile;

use crate::api::banzai::Banzai;
use crate::tests::helpers::win32_maps_folder;
use crate::{emu32, emu64};

#[test]
fn test_banzai_initially_empty() {
    let emu = emu64();

    assert_eq!(emu.banzai.get_params("UnknownCall"), -1);
}

#[test]
fn test_banzai_add_function() {
    let mut emu = emu64();

    // Add a function to banazai list
    emu.banzai_add("CreateFileA", 7);

    // Verify it was added by trying to get params
    let params = emu.banzai.get_params("CreateFileA");
    assert_eq!(params, 7, "Parameter count should match");
}

#[test]
fn test_banzai_add_multiple_functions() {
    let mut emu = emu64();

    // Add multiple functions
    emu.banzai_add("CreateFileA", 7);
    emu.banzai_add("ReadFile", 5);
    emu.banzai_add("WriteFile", 5);

    // Verify all were added
    assert_eq!(emu.banzai.get_params("CreateFileA"), 7);
    assert_eq!(emu.banzai.get_params("ReadFile"), 5);
    assert_eq!(emu.banzai.get_params("WriteFile"), 5);
}

#[test]
fn test_banzai_different_param_counts() {
    let mut emu = emu64();

    // Add functions with different parameter counts
    emu.banzai_add("Func0", 0);
    emu.banzai_add("Func1", 1);
    emu.banzai_add("Func10", 10);

    assert_eq!(emu.banzai.get_params("Func0"), 0);
    assert_eq!(emu.banzai.get_params("Func1"), 1);
    assert_eq!(emu.banzai.get_params("Func10"), 10);
}

#[test]
fn test_banzai_add_overwrite() {
    let mut emu = emu64();

    // Add a function
    emu.banzai_add("TestFunc", 5);
    assert_eq!(emu.banzai.get_params("TestFunc"), 5);

    // Add it again with different param count
    emu.banzai_add("TestFunc", 10);

    // Should be overwritten
    assert_eq!(
        emu.banzai.get_params("TestFunc"),
        10,
        "Parameter count should be updated"
    );
}

#[test]
fn test_banzai_32bit_mode() {
    let mut emu = emu32();

    emu.banzai_add("TestFunc32", 3);
    assert_eq!(
        emu.banzai.get_params("TestFunc32"),
        3,
        "Banzai should work in 32-bit mode"
    );
}

#[test]
fn test_banzai_enable_works() {
    let mut emu = emu64();

    // enable_banzai and disable_banzai exist and should not panic
    emu.enable_banzai();
    emu.disable_banzai();
}

#[test]
fn test_banzai_add_clears_previous() {
    let mut emu = emu64();

    emu.banzai_add("Func1", 1);
    emu.banzai_add("Func2", 2);

    assert_eq!(emu.banzai.get_params("Func1"), 1);
    assert_eq!(emu.banzai.get_params("Func2"), 2);
}

#[test]
fn test_banzai_load_headerless_csv() {
    let mut banzai = Banzai::new();
    banzai.add("CreateFileA", 99);
    banzai.load(&format!("{}banzai.csv", win32_maps_folder()));
    assert_eq!(banzai.get_params("??2@YAPAXI@Z"), 1); // First row is data.
    assert_eq!(banzai.get_params("CreateFileA"), 7);
    assert_eq!(banzai.get_params("NtFlushBuffersFileEx"), 5);
    assert_eq!(banzai.get_params("NtSetSystemEnvironmentValueEx"), 5);
    assert_eq!(banzai.get_params("NtSetInformationProcess"), 4);

    let mut csv = NamedTempFile::new().unwrap();
    write!(csv, "first,2\nbad,invalid\nfirst,4\nlast,0\n").unwrap();
    banzai.load(csv.path().to_str().unwrap());
    assert_eq!(banzai.get_params("first"), 4); // Later entries still win.
    assert_eq!(banzai.get_params("last"), 0);
    assert_eq!(banzai.get_params("bad"), -1); // Invalid counts are not zero-arg APIs.
    banzai.add("first", 6);
    assert_eq!(banzai.get_params("first"), 6); // Runtime overrides still work.
}
