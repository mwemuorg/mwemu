use crate::emu::Emu;
use crate::emu::object_handle::HeapHandle;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers::{call_winapi32, call_winapi64};
use crate::winapi::common::heap;
use crate::windows::constants::{HEAP_REALLOC_IN_PLACE_ONLY, HEAP_ZERO_MEMORY};
use crate::{emu32, emu64};
use itertools::iproduct;

fn prepare(is64: bool, guard: bool) -> (Emu, u64, u64) {
    let mut emu = if is64 { emu64() } else { emu32() };
    emu.cfg.memory_guard = guard;
    if guard {
        emu.memory_guard_init();
    }
    emu.cfg.console_enabled = false;
    let mut handles = Vec::new();
    for _ in 0..2 {
        let arena = emu.create_heap_arena(0x10000, 0x10000, 0).unwrap();
        let base = emu.heap_arenas[arena].base();
        let key = emu
            .handle_management
            .insert_heap_handle(HeapHandle::new(0, 0x10000, 0x10000, arena));
        emu.handle_management.set_heap_base_addr(key, base);
        handles.push(base);
    }
    (emu, handles[0], handles[1])
}

#[test]
fn heap_ownership_and_zero_initialization_are_shared_across_modes() {
    for (is64, guard) in iproduct!([false, true], [false, true]) {
        let (mut emu, first, second) = prepare(is64, guard);
        let address = heap::allocate(&mut emu, first, 257, HEAP_ZERO_MEMORY).unwrap();
        assert_eq!(emu.maps.read_byte(address + 256), Some(0));
        assert_eq!(heap::owned_size(&emu, first, address), Some(257));
        assert_eq!(heap::owned_size(&emu, second, address), None);
        assert!(!heap::heap_free(&mut emu, second, address));
        assert!(
            heap::reallocate(&mut emu, second, address, 512, HEAP_REALLOC_IN_PLACE_ONLY).is_none()
        );
        assert!(emu.maps.fill_range(address, 0xab, 257));
        let grown = heap::reallocate(&mut emu, first, address, 700, HEAP_ZERO_MEMORY).unwrap();
        assert_eq!(emu.maps.read_byte(grown + 256), Some(0xab));
        for offset in [257, 511, 699] {
            assert_eq!(emu.maps.read_byte(grown + offset), Some(0));
        }
        assert!(heap::heap_free(&mut emu, first, grown));
        assert!(!heap::heap_free(&mut emu, first, grown));
        let fresh = heap::allocate(&mut emu, first, 700, HEAP_ZERO_MEMORY).unwrap();
        assert_eq!(emu.maps.read_byte(fresh), Some(0));
    }
}

#[test]
fn failed_reallocation_keeps_original_allocation_and_bytes() {
    for is64 in [false, true] {
        let (mut emu, handle, _) = prepare(is64, false);
        let before = heap::allocate(&mut emu, handle, 256, 0).unwrap();
        let old = heap::allocate(&mut emu, handle, 256, 0).unwrap();
        let _after = heap::allocate(&mut emu, handle, 256, 0).unwrap();
        emu.maps.write_dword(old, 0x12345678);
        assert!(heap::heap_free(&mut emu, handle, before));
        assert!(heap::reallocate(&mut emu, handle, old, 512, HEAP_REALLOC_IN_PLACE_ONLY).is_none());
        assert_eq!(heap::owned_size(&emu, handle, old), Some(256));
        emu.maps
            .get_mem_by_addr_mut(old)
            .unwrap()
            .set_permission_range(old, 256, Permission::NONE);
        assert!(heap::reallocate(&mut emu, handle, old, 512, 0).is_none());
        assert_eq!(heap::owned_size(&emu, handle, old), Some(256));
        emu.maps
            .get_mem_by_addr_mut(old)
            .unwrap()
            .set_permission_range(old, 256, Permission::READ_WRITE);
        assert_eq!(emu.maps.read_dword(old), Some(0x12345678));
        let moved = heap::reallocate(&mut emu, handle, old, 512, 0).unwrap();
        assert_eq!(moved, before);
        assert_eq!(emu.maps.read_dword(moved), Some(0x12345678));
        assert_eq!(heap::owned_size(&emu, handle, old), None);
    }
}

#[test]
fn destroyed_heap_cannot_allocate_or_match_wrapped_addresses() {
    let (mut emu, handle, _) = prepare(true, false);
    let address = heap::allocate(&mut emu, handle, 256, 0).unwrap();
    let (arena, _) = heap::alloc_context(&emu, handle).unwrap();
    assert!(!emu.heap_arenas[arena].check_fragment_exists(address + 0x1_0000_0000));
    assert_eq!(
        emu.heap_arenas[arena].allocation_size(address + 0x1_0000_0000),
        None
    );
    emu.destroy_heap_arena(arena);
    assert!(heap::allocate(&mut emu, handle, 256, 0).is_none());
    assert!(heap::owned_size(&emu, handle, address).is_none());
}

#[test]
fn native_snapshot_preserves_heap_owners_and_allocation_metadata() {
    for (is64, guard) in iproduct!([false, true], [false, true]) {
        let (mut emu, first, second) = prepare(is64, guard);
        let address = heap::allocate(&mut emu, first, 257, 0).unwrap();
        if !is64 {
            assert!(address <= u32::MAX as u64);
        }
        emu.maps.write_dword(address, 0x12345678);
        let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(heap::owned_size(&restored, first, address), Some(257));
        assert!(!heap::heap_free(&mut restored, second, address));
        let next = heap::allocate(&mut restored, first, 257, 0).unwrap();
        assert_ne!(next, address);
        assert_eq!(restored.maps.read_dword(address), Some(0x12345678));
        assert!(heap::heap_free(&mut restored, first, address));
        assert_eq!(heap::owned_size(&emu, first, address), Some(257));
        let (arena, _) = heap::alloc_context(&restored, first).unwrap();
        restored.destroy_heap_arena(arena);
        assert_eq!(heap::owned_size(&restored, first, next), None);
    }
}

#[test]
fn heap_destroy_dispatch_releases_owned_blocks_in_both_modes() {
    for (is64, guard) in iproduct!([false, true], [false, true]) {
        let (mut emu, handle, _) = prepare(is64, guard);
        let address = heap::allocate(&mut emu, handle, 257, 0).unwrap();
        let destroy = |emu: &mut Emu| {
            assert!(heap::dispatch("HeapDestroy", emu));
        };
        let result = if is64 {
            call_winapi64(&mut emu, destroy, &[handle])
        } else {
            call_winapi32(&mut emu, destroy, &[handle as u32]) as u64
        };
        assert_eq!(result, 1);
        assert_eq!(heap::owned_size(&emu, handle, address), None);
        assert!(heap::allocate(&mut emu, handle, 32, 0).is_none());
    }
}
