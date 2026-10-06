//! `Maps::unmap_range` follows Linux `munmap`: whole maps inside the range
//! disappear, maps that only partially overlap are trimmed, and a map that
//! contains the range is split in two with its bytes intact.

use crate::maps::Maps;
use crate::maps::mem64::Permission;

fn maps_with(base: u64, size: u64) -> Maps {
    let mut maps = Maps::default();
    maps.is_64bits = true;
    maps.create_map("m", base, size, Permission::READ_WRITE)
        .expect("create");
    for i in 0..size {
        maps.write_byte(base + i, (i & 0xff) as u8);
    }
    maps
}

#[test]
fn unmap_whole_map() {
    let mut maps = maps_with(0x10000, 0x3000);
    maps.unmap_range(0x10000, 0x3000);
    assert!(!maps.is_mapped(0x10000));
    assert!(!maps.is_mapped(0x12fff));
}

#[test]
fn unmap_head_moves_base() {
    let mut maps = maps_with(0x10000, 0x3000);
    maps.unmap_range(0x10000, 0x1000);
    assert!(!maps.is_mapped(0x10fff));
    assert!(maps.is_mapped(0x11000));
    assert_eq!(maps.read_byte(0x11000), Some(0x00));
    assert_eq!(maps.read_byte(0x12fff), Some(0xff));
    assert!(maps.is_range_free(0x10000, 0x1000));
}

#[test]
fn unmap_tail_truncates() {
    let mut maps = maps_with(0x10000, 0x3000);
    maps.unmap_range(0x12000, 0x1000);
    assert!(maps.is_mapped(0x11fff));
    assert!(!maps.is_mapped(0x12000));
    assert_eq!(maps.read_byte(0x11fff), Some(0xff));
}

#[test]
fn unmap_hole_splits_and_keeps_bytes() {
    let mut maps = maps_with(0x10000, 0x3000);
    maps.unmap_range(0x11000, 0x1000);
    assert!(maps.is_mapped(0x10fff));
    assert!(!maps.is_mapped(0x11000));
    assert!(!maps.is_mapped(0x11fff));
    assert!(maps.is_mapped(0x12000));
    assert_eq!(maps.read_byte(0x12000), Some(0x00));
    assert_eq!(maps.read_byte(0x12fff), Some(0xff));
    // The hole can be mapped again, under a name that does not collide.
    let name = maps.unique_map_name("m");
    assert_ne!(name, "m");
    maps.create_map(&name, 0x11000, 0x1000, Permission::READ_WRITE)
        .expect("re-map the hole");
    assert!(maps.is_mapped(0x11800));
}

#[test]
fn unmap_range_spanning_two_maps() {
    let mut maps = maps_with(0x10000, 0x2000);
    maps.create_map("n", 0x12000, 0x2000, Permission::READ_WRITE)
        .expect("create");
    maps.unmap_range(0x11000, 0x2000);
    assert!(maps.is_mapped(0x10fff));
    assert!(!maps.is_mapped(0x11000));
    assert!(!maps.is_mapped(0x12fff));
    assert!(maps.is_mapped(0x13000));
}
