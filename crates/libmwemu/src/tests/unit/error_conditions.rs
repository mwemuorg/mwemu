use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
// test error conditions and edge cases
pub fn error_conditions() {
    helpers::setup();

    let mut emu = emu64();
    // Don't call init to avoid DLL loading issues

    // Test invalid memory access with banzai mode
    emu.maps.set_banzai(true);
    assert!(emu.maps.read_dword(0x999999).is_none());
    assert!(!emu.maps.write_dword(0x999999, 0x12345678));

    // Test reading from unallocated memory
    assert!(emu.maps.read_qword(0x123456789).is_none());
    assert!(!emu.maps.write_qword(0x123456789, 0xDEADBEEF));

    // Test zero-sized memory operations
    let base = 0x20000;
    emu.maps
        .create_map("zero_test", base, 0x1000, Permission::READ_WRITE_EXECUTE)
        .unwrap();

    // Test reading/writing at exact boundaries
    assert!(emu.maps.write_dword(base + 0x1000 - 4, 0x12345678));
    assert!(emu.maps.read_dword(base + 0x1000 - 4).is_some());

    // Test one byte past boundary should fail with banzai mode
    assert!(!emu.maps.write_dword(base + 0x1000 - 3, 0x12345678));

    // Test string operations with boundaries
    let long_string = "A".repeat(100);
    emu.maps.write_string(base, &long_string);
    let read_string = emu.maps.read_string(base);
    assert_eq!(read_string, long_string);
}
