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
    assert!(!test_map.permission().can_execute());

    // Test duplicate map creation should fail
    let result2 = emu
        .maps
        .create_map("test_map", base, size, Permission::READ_WRITE);
    assert!(result2.is_err());
}

#[test]
fn scalars_and_buffers_span_every_contiguous_map() {
    let mut emu = emu64();
    let maps = &mut emu.maps;
    let start = 0x1000;
    for (name, offset, size, permission) in [
        ("first", 0, 3, Permission::READ_WRITE),
        ("code", 3, 2, Permission::READ_WRITE_EXECUTE),
        ("last", 5, 16, Permission::READ_WRITE),
    ] {
        maps.create_map(name, start + offset, size, permission)
            .unwrap();
    }
    let mut writes = Vec::new();
    maps.drain_code_writes(&mut writes);
    let value = u128::from_le_bytes(*b"0123456789abcdef");
    assert!(maps.write_128bits_le(start + 1, value));
    assert_eq!(maps.read_128bits_le(start + 1), Some(value));
    assert_eq!(maps.read_128bits_be(start + 1), Some(value.swap_bytes()));
    assert_eq!(
        maps.try_read_bytes(start + 1, 16).as_deref(),
        Some(b"0123456789abcdef".as_slice())
    );
    maps.drain_code_writes(&mut writes);
    assert_eq!(writes, [(start + 3, 2)]);

    maps.get_mem_mut("last").set_permission(Permission::READ);
    assert!(!maps.write_qword(start + 1, u64::MAX));
    assert_eq!(
        maps.read_qword(start + 1),
        Some(u64::from_le_bytes(*b"01234567"))
    );
    assert!(!maps.has_code_writes());
    maps.free("code");
    assert!(maps.try_read_bytes(start + 1, 8).is_none());
    assert!(!maps.write_qword(start + 1, 0));
    assert_eq!(maps.read_word(start + 1), Some(u16::from_le_bytes(*b"01")));
}
