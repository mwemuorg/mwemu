use crate::maps::mem64::Permission;
use crate::{tests::helpers, *};

#[test]
// test memory map operations and edge cases
pub fn memory_map_operations() {
    helpers::setup();

    let mut emu = emu64();
    // Don't call init to avoid DLL loading issues

    // Test multiple memory maps
    emu.maps
        .create_map("map1", 0x10000, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps
        .create_map("map2", 0x20000, 0x2000, Permission::READ_WRITE)
        .unwrap();
    emu.maps
        .create_map("map3", 0x30000, 0x1000, Permission::READ_WRITE)
        .unwrap();

    // Test map existence
    assert!(emu.maps.exists_mapname("map1"));
    assert!(emu.maps.exists_mapname("map2"));
    assert!(emu.maps.exists_mapname("map3"));
    assert!(!emu.maps.exists_mapname("nonexistent"));

    // Test memory allocation checks
    assert!(emu.maps.is_allocated(0x10000));
    assert!(emu.maps.is_allocated(0x10500));
    assert!(emu.maps.is_allocated(0x10FFF));
    assert!(!emu.maps.is_allocated(0x11000));

    assert!(emu.maps.is_allocated(0x20000));
    assert!(emu.maps.is_allocated(0x21FFF));
    assert!(!emu.maps.is_allocated(0x22000));

    // Test getting map by name
    let map1 = emu.maps.get_map_by_name("map1");
    assert!(map1.is_some());
    assert_eq!(map1.unwrap().get_base(), 0x10000);

    let nonexistent = emu.maps.get_map_by_name("nonexistent");
    assert!(nonexistent.is_none());

    // Test memory size queries
    let size1 = emu.maps.get_mem_size(0x10000);
    assert!(size1.is_some());

    let size_invalid = emu.maps.get_mem_size(0x99999);
    assert!(size_invalid.is_none());

    // Test cross-map operations
    assert!(emu.maps.write_dword(0x10000, 0x11111111));
    assert!(emu.maps.write_dword(0x20000, 0x22222222));
    assert!(emu.maps.write_dword(0x30000, 0x33333333));

    assert_eq!(emu.maps.read_dword(0x10000).unwrap(), 0x11111111);
    assert_eq!(emu.maps.read_dword(0x20000).unwrap(), 0x22222222);
    assert_eq!(emu.maps.read_dword(0x30000).unwrap(), 0x33333333);
}
