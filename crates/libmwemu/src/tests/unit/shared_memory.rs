use crate::emu64;
use crate::maps::Maps;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;

const FIRST: u64 = 0x4000_0000;
const SECOND: u64 = 0x5000_0000;
const PAGE: u64 = 0x1000;

fn views() -> Maps {
    let mut maps = Maps::default();
    let storage = maps.create_shared_storage(vec![0; 3 * PAGE as usize]);
    maps.create_shared_view(
        "first",
        FIRST,
        3 * PAGE,
        storage,
        0,
        Permission::READ_WRITE_EXECUTE,
    )
    .unwrap();
    maps.create_shared_view(
        "second",
        SECOND,
        2 * PAGE,
        storage,
        PAGE,
        Permission::READ_WRITE,
    )
    .unwrap();
    maps
}

#[test]
fn views_share_scalar_bulk_and_direct_writes_with_independent_protection() {
    let mut maps = views();
    let mut writes = Vec::new();
    maps.drain_code_writes(&mut writes);
    maps.get_mem_by_addr_mut(SECOND)
        .unwrap()
        .write_dword(SECOND, 0x12345678);
    assert_eq!(maps.read_dword(FIRST + PAGE), Some(0x12345678));
    assert!(maps.has_code_writes());
    maps.drain_code_writes(&mut writes);
    assert!(writes.contains(&(0, u64::MAX)));
    assert!(!maps.has_code_writes());
    maps.get_mem_by_addr_mut(FIRST)
        .unwrap()
        .set_permission_range(FIRST + PAGE, PAGE, Permission::READ);
    assert!(!maps.write_byte(FIRST + PAGE, 0));
    assert!(maps.write_bytes(SECOND, &[1, 2, 3, 4]));
    assert_eq!(maps.read_dword(FIRST + PAGE), Some(0x04030201));
    assert!(maps.fill_range(SECOND + PAGE, 0xab, 16));
    assert_eq!(
        &maps.get_mem("first").get_bytes()[2 * PAGE as usize..2 * PAGE as usize + 4],
        &[0xab; 4]
    );
    maps.dealloc(FIRST);
    assert_eq!(maps.read_dword(SECOND), Some(0x04030201));
}

#[test]
fn partial_unmap_keeps_offsets_and_aliasing_of_the_remaining_view() {
    let mut maps = views();
    maps.unmap_range(FIRST + PAGE, PAGE);
    assert!(!maps.is_mapped(FIRST + PAGE));
    assert!(maps.write_dword(SECOND + PAGE, 0x12345678));
    assert_eq!(maps.read_dword(FIRST + 2 * PAGE), Some(0x12345678));
    assert!(maps.write_byte(FIRST, 0x42));
    assert!(maps.mem_test());
}

#[test]
fn clone_and_native_snapshot_preserve_aliases_without_sharing_with_the_source() {
    let mut emu = emu64();
    emu.maps = views();
    emu.maps.write_dword(SECOND, 1);
    let mut clone = emu.maps.clone();
    assert!(clone.has_code_writes());
    clone.write_dword(SECOND, 2);
    assert_eq!(clone.read_dword(FIRST + PAGE), Some(2));
    assert_eq!(emu.maps.read_dword(FIRST + PAGE), Some(1));
    let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
    restored.maps.write_dword(SECOND, 3);
    assert_eq!(restored.maps.read_dword(FIRST + PAGE), Some(3));
    assert_eq!(emu.maps.read_dword(FIRST + PAGE), Some(1));
}

#[test]
fn overlapping_alias_copy_reads_the_source_before_writing() {
    let mut maps = views();
    maps.write_bytes(SECOND, &[1, 2, 3, 4, 5]);
    assert!(maps.copy_range(FIRST + PAGE + 1, SECOND, 4));
    assert_eq!(maps.read_bytes(SECOND, 5).as_ref(), &[1, 1, 2, 3, 4]);
}
