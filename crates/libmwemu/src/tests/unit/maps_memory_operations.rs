use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
// test memory management operations
pub fn maps_memory_operations() {
    helpers::setup();

    let mut emu = emu64();
    // with no init call

    // Test memory allocation
    let base = 0x10000;
    let size = 0x1000;
    let result = emu
        .maps
        .create_map("test_map", base, size, Permission::READ_WRITE);
    assert!(result.is_ok());

    // Test memory exists
    assert!(emu.maps.is_allocated(base));
    assert!(emu.maps.exists_mapname("test_map"));

    // Test memory read/write operations
    assert!(emu.maps.write_dword(base, 0xDEADBEEF));
    assert_eq!(emu.maps.read_dword(base).unwrap(), 0xDEADBEEF);

    // Test qword operations
    assert!(emu.maps.write_qword(base + 8, 0x123456789ABCDEF0));
    assert_eq!(emu.maps.read_qword(base + 8).unwrap(), 0x123456789ABCDEF0);

    // Test byte operations
    assert!(emu.maps.write_byte(base + 16, 0xAB));
    assert_eq!(emu.maps.read_byte(base + 16).unwrap(), 0xAB);

    // Test word operations
    assert!(emu.maps.write_word(base + 18, 0x1234));
    assert_eq!(emu.maps.read_word(base + 18).unwrap(), 0x1234);

    // Test boundary conditions - should fail with banzai mode
    emu.maps.set_banzai(true);
    assert!(!emu.maps.write_dword(base + size, 0x12345678));
    assert!(emu.maps.read_dword(base + size).is_none());

    // Test string operations
    let test_str = "Hello World";
    emu.maps.write_string(base + 32, test_str);
    assert_eq!(emu.maps.read_string(base + 32), test_str);
    let test_map = emu
        .maps
        .get_map_by_name("test_map")
        .expect("Fail to get map");
    assert_eq!(test_map.permission().can_execute(), false);

    // Test duplicate map creation should fail
    let result2 = emu
        .maps
        .create_map("test_map", base, size, Permission::READ_WRITE);
    assert!(result2.is_err());
}
