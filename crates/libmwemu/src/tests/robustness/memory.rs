//! Memory (`Maps`) robustness — hostile addresses must never panic.

use crate::emu64;
use crate::maps::mem64::Permission;

#[test]
fn read_unmapped_returns_none() {
    let emu = emu64();
    let bad = 0x0000_dead_beef_0000;
    assert_eq!(emu.maps.read_qword(bad), None);
    assert_eq!(emu.maps.read_dword(bad), None);
    assert_eq!(emu.maps.read_word(bad), None);
    assert_eq!(emu.maps.read_byte(bad), None);
}

#[test]
fn write_unmapped_returns_false_no_panic() {
    let mut emu = emu64();
    let bad = 0x0000_dead_beef_0000;
    assert!(!emu.maps.write_qword(bad, 0x4141_4141_4141_4141));
    assert!(!emu.maps.write_dword(bad, 0x4141_4141));
    assert!(!emu.maps.write_byte(bad, 0x41));
}

#[test]
fn create_map_read_write_roundtrip() {
    let mut emu = emu64();
    let base = 0x0040_0000;
    emu.maps
        .create_map("rob_rw", base, 0x1000, Permission::READ_WRITE)
        .expect("create_map");
    assert!(emu.maps.write_qword(base, 0xcafe_babe_dead_beef));
    assert_eq!(emu.maps.read_qword(base), Some(0xcafe_babe_dead_beef));
    assert!(emu.maps.write_dword(base + 0x10, 0x1234_5678));
    assert_eq!(emu.maps.read_dword(base + 0x10), Some(0x1234_5678));
}

#[test]
fn read_past_map_boundary_no_panic() {
    // A qword read that straddles or exceeds the map end must not panic.
    let mut emu = emu64();
    let base = 0x0050_0000;
    emu.maps
        .create_map("rob_edge", base, 8, Permission::READ_WRITE)
        .expect("create_map");
    let _ = emu.maps.read_qword(base + 4); // straddles the end
    let _ = emu.maps.read_qword(base + 4096); // fully beyond
    let _ = emu.maps.read_byte(base + 4096);
}

#[test]
fn is_mapped_and_overlaps() {
    let mut emu = emu64();
    let base = 0x0060_0000;
    emu.maps
        .create_map("rob_map", base, 0x1000, Permission::READ_WRITE)
        .expect("create_map");
    assert!(emu.maps.is_mapped(base));
    assert!(emu.maps.is_mapped(base + 0x100));
    assert!(!emu.maps.is_mapped(0x0000_9999_9999));
    assert!(emu.maps.overlaps(base, 0x100));
    assert!(!emu.maps.overlaps(0x0000_9999_9999, 0x100));
}

#[test]
fn duplicate_map_name_does_not_panic() {
    let mut emu = emu64();
    emu.maps
        .create_map("rob_dup", 0x0070_0000, 0x100, Permission::READ_WRITE)
        .expect("first create");
    // Second create with a colliding name/region: whatever the policy, it must
    // return a Result (Ok or Err), never panic.
    let _ = emu
        .maps
        .create_map("rob_dup", 0x0070_0000, 0x100, Permission::READ_WRITE);
}
