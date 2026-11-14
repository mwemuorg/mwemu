use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::memory::*;

use crate::emu::Emu;
use crate::emu::object_handle::HeapHandle;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers::{self, call_winapi32, call_winapi64};
use crate::winapi::common::heap;
use crate::{emu32, emu64};
use itertools::iproduct;

const HEAP_SIZE_ERROR: u64 = u64::MAX;

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

// 32-bit counterpart of the kishou HeapAlloc regression. The 32-bit init path
// (`init_win32_mem32`) never set up `heap_management`, so a small `HeapAlloc`
// panicked on `unwrap()`. `Emu::heap_mut()` now lazily builds the arena.
#[test]
fn heap_alloc_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    // HeapAlloc(hHeap, dwFlags, dwBytes) via the stdcall calling convention.
    // Small allocation → managed heap path (< 0x8000).
    let p1 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[heap, HEAP_ZERO_MEMORY as u32, 0x100],
    ) as u64;
    assert!(p1 != 0, "HeapAlloc(0x100) returned NULL");
    assert!(
        emu.maps.is_mapped(p1),
        "HeapAlloc(0x100) pointer not mapped"
    );
    emu.maps.write_dword(p1, 0xcafebabe);
    assert_eq!(emu.maps.read_dword(p1).unwrap(), 0xcafebabe);

    // A second allocation must land somewhere else.
    let p2 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x100]) as u64;
    assert!(p2 != 0, "second HeapAlloc returned NULL");
    assert!(p2 != p1, "two allocations returned the same pointer");

    // Large allocation → dedicated map path (>= 0x8000).
    let big =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x20000]) as u64;
    assert!(big != 0, "large HeapAlloc returned NULL");
    assert!(emu.maps.is_mapped(big), "large HeapAlloc not mapped");
}

#[test]
fn heap_realloc_small_to_small_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x100]) as u64;
    assert!(p1 != 0);
    emu.maps.write_dword(p1, 0xdeadbeef);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, 0, p1 as u32, 0x400],
    ) as u64;
    assert!(p2 != 0, "HeapReAlloc returned NULL");
    assert_eq!(emu.maps.read_dword(p2).unwrap(), 0xdeadbeef);
}

#[test]
fn heap_realloc_small_to_large_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x100]) as u64;
    assert!(p1 != 0);
    emu.maps.write_dword(p1, 0x11223344);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, 0, p1 as u32, 0x20000],
    ) as u64;
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_dword(p2).unwrap(), 0x11223344);
}

#[test]
fn heap_realloc_large_to_large_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x20000]) as u64;
    assert!(p1 != 0);
    emu.maps.write_dword(p1 + 0x100, 0xaabbccdd);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, 0, p1 as u32, 0x30000],
    ) as u64;
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_dword(p2 + 0x100).unwrap(), 0xaabbccdd);
}

#[test]
fn heap_realloc_zero_memory_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x100]) as u64;
    assert!(p1 != 0);
    for i in 0..0x100 {
        emu.maps.write_byte(p1 + i, 0xab);
    }

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, HEAP_ZERO_MEMORY as u32, p1 as u32, 0x20000],
    ) as u64;
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_byte(p2).unwrap(), 0xab);
    for i in 0x100..0x200 {
        assert_eq!(emu.maps.read_byte(p2 + i).unwrap(), 0, "byte at +{:#x}", i);
    }
}

#[test]
fn heap_realloc_invalid_pointer_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn heap_realloc_in_place_shrink_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x20000]) as u64;
    assert!(p1 != 0);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, HEAP_REALLOC_IN_PLACE_ONLY as u32, p1 as u32, 0x100],
    ) as u64;
    assert_eq!(p1, p2);
}

#[test]
fn heap_realloc_in_place_grow_fails_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x100]) as u64;
    assert!(p1 != 0);
    let _blocker =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[heap, 0, 0x100]);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, HEAP_REALLOC_IN_PLACE_ONLY as u32, p1 as u32, 0x20000],
    );
    assert_eq!(ret, 0);
}

#[test]
fn heap_realloc_zero_size_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, 0, 0x1000, 0],
    );
    assert_eq!(ret, 0);
}

#[test]
fn heap_realloc_null_ptr_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[heap, 0, 0, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn ntdll_rtl_realloc_32() {
    helpers::setup();
    let mut emu = emu32();
    let heap = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlAllocateHeap,
        &[heap, 0, 0x100],
    ) as u64;
    assert!(p1 != 0, "RtlAllocateHeap returned NULL");
    emu.maps.write_dword(p1, 0x99887766);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlReAllocateHeap,
        &[heap, 0, p1 as u32, 0x400],
    ) as u64;
    assert!(p2 != 0, "RtlReAllocateHeap returned NULL");
    assert_eq!(emu.maps.read_dword(p2).unwrap(), 0x99887766);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlReAllocateHeap,
        &[heap, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

// GetProcessHeap must return a non-zero handle (the slab slot 0 is
// reserved so the first real handle starts at key 1).
#[test]
fn get_process_heap_returns_handle_32() {
    helpers::setup();
    let mut emu = emu32();

    let proc_heap =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]) as u64;
    assert_ne!(proc_heap, 0, "GetProcessHeap returned NULL");

    let p = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[proc_heap as u32, 0, 0x100],
    ) as u64;
    assert_ne!(p, 0, "HeapAlloc via GetProcessHeap returned NULL");
}

// Private-heap lifecycle: create -> alloc -> free -> alloc -> destroy, plus
// zero-size HeapCreate clamping (must not panic).
#[test]
fn heap_lifecycle_32() {
    helpers::setup();
    let mut emu = emu32();

    let heap = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapCreate,
        &[0, 0x1000, 0x10000],
    ) as u64;
    assert_ne!(heap, 0, "HeapCreate returned NULL");

    let p1 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[heap as u32, 0, 0x100],
    ) as u64;
    assert_ne!(p1, 0, "HeapAlloc on private heap returned NULL");
    emu.maps.write_dword(p1, 0xfeedface);

    let freed = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapFree,
        &[heap as u32, 0, p1 as u32],
    ) as u64;
    assert_eq!(freed, 1, "HeapFree on private heap failed");

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[heap as u32, 0, 0x100],
    ) as u64;
    assert_ne!(p2, 0, "HeapAlloc after free on private heap returned NULL");

    let destroyed =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapDestroy, &[heap as u32]) as u64;
    assert_eq!(destroyed, 1, "HeapDestroy failed");
    let again =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapDestroy, &[heap as u32]) as u64;
    assert_eq!(again, 0, "second HeapDestroy must fail");

    let zero = helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapCreate, &[0, 0, 0]) as u64;
    assert_ne!(zero, 0, "HeapCreate(0,0,0) must clamp and succeed");
    let destroyed =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapDestroy, &[zero as u32]) as u64;
    assert_eq!(destroyed, 1, "HeapDestroy of clamped heap failed");
}

// Growing a block whose predecessor is free must move it backward in place
// (O1Heap backward expansion) while preserving the payload.
#[test]
fn heap_realloc_backward_move_preserves_data_32() {
    helpers::setup();
    let mut emu = emu32();

    let proc = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]) as u64;
    assert_ne!(proc, 0);

    let a = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[proc as u32, 0, 0x8000],
    ) as u64;
    let b = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[proc as u32, 0, 0x100],
    ) as u64;
    let _c = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[proc as u32, 0, 0x100],
    ) as u64;
    assert!(a != 0 && b != 0);

    emu.maps.write_dword(b, 0xabcd1234);
    let freed = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapFree,
        &[proc as u32, 0, a as u32],
    ) as u64;
    assert_eq!(freed, 1);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[proc as u32, 0, b as u32, 0x4000],
    ) as u64;
    assert_eq!(
        p2, a,
        "backward expansion must reuse the freed predecessor address"
    );
    assert_eq!(
        emu.maps.read_dword(p2).unwrap(),
        0xabcd1234,
        "payload must survive the backward move"
    );
}

// Growing a block whose both neighbors are in use must move it (allocate +
// copy + free) while preserving the payload.
#[test]
fn heap_realloc_fallback_move_preserves_data_32() {
    helpers::setup();
    let mut emu = emu32();

    let proc = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]) as u64;
    assert_ne!(proc, 0);

    let _a = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[proc as u32, 0, 0x100],
    ) as u64;
    let b = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[proc as u32, 0, 0x100],
    ) as u64;
    let _big = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[proc as u32, 0, 0x8000],
    ) as u64;
    assert_ne!(b, 0);

    emu.maps.write_dword(b, 0x55667788);
    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[proc as u32, 0, b as u32, 0x4000],
    ) as u64;
    assert_ne!(p2, b, "fallback must relocate the block");
    assert_eq!(
        emu.maps.read_dword(p2).unwrap(),
        0x55667788,
        "payload must survive the fallback move"
    );
}

#[test]
fn heap_handle_is_real_address_32() {
    helpers::setup();
    let mut emu = emu32();

    let proc = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]) as u64;
    assert_ne!(proc, 0, "GetProcessHeap must not return NULL");
    assert!(
        proc > 0xFFFF,
        "GetProcessHeap must return a real mapped address, not a bare slab index (got 0x{:x})",
        proc
    );
    assert!(
        emu.maps.read_dword(proc).is_some(),
        "GetProcessHeap address 0x{:x} must be readable (mapped memory)",
        proc
    );

    let proc2 = helpers::call_winapi32(&mut emu, winapi32::kernel32::GetProcessHeap, &[]) as u64;
    assert_eq!(proc, proc2, "GetProcessHeap must be stable across calls");

    let heap = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapCreate,
        &[0, 0x1000, 0x10000],
    ) as u64;
    assert_ne!(heap, 0, "HeapCreate must not return NULL");
    assert!(
        heap > 0xFFFF,
        "HeapCreate must return a real mapped address, not a bare slab index (got 0x{:x})",
        heap
    );
    assert!(
        emu.maps.read_dword(heap).is_some(),
        "HeapCreate address 0x{:x} must be readable (mapped memory)",
        heap
    );
    assert_ne!(
        heap, proc,
        "HeapCreate must return a different address than the process heap"
    );

    let p = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[heap as u32, 0, 0x100],
    ) as u64;
    assert_ne!(p, 0, "HeapAlloc with real-address handle must work");

    let destroyed =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapDestroy, &[heap as u32]);
    assert_eq!(
        destroyed, 1,
        "HeapDestroy with real-address handle must work"
    );
}

// ---- newly mirrored 32-bit APIs ----
#[test]
fn global_alloc_free_32() {
    helpers::setup();
    let mut emu = emu32();

    // GlobalAlloc(GMEM_ZEROINIT, 0x80)
    let p = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::GlobalAlloc,
        &[GMEM_ZEROINIT, 0x80],
    ) as u64;
    assert!(
        p != 0 && emu.maps.is_mapped(p),
        "GlobalAlloc returned an unmapped pointer"
    );
    assert_eq!(
        emu.maps.read_dword(p).unwrap(),
        0,
        "GMEM_ZEROINIT not honoured"
    );

    // HeapSize(heap, 0, p) reports the managed size.
    let sz = helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapSize, &[0, 0, p as u32]);
    assert!(sz >= 0x80, "HeapSize {} smaller than the request", sz);

    let r = helpers::call_winapi32(&mut emu, winapi32::kernel32::GlobalFree, &[p as u32]);
    assert_eq!(r, 0, "GlobalFree should return NULL on success");
}

// Regression test for the heap bug reported by kishou: a small `HeapAlloc`
// panicked because `heap_management` was `None` unless the 64-bit normal-mode
// init had run (it is left `None` by the 32-bit path, by SSDT/syscall mode,
// and right after deserialization). `Emu::heap_mut()` now creates the arena
// lazily, so a bare `emu64()` can allocate without any prior init.
#[test]
fn heap_alloc_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    // HeapAlloc(hHeap, dwFlags, dwBytes) via the x64 calling convention.
    // Small allocation → managed heap path (< 0x8000).
    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[heap, HEAP_ZERO_MEMORY, 0x100],
    );
    assert!(p1 != 0, "HeapAlloc(0x100) returned NULL");
    assert!(
        emu.maps.is_mapped(p1),
        "HeapAlloc(0x100) pointer not mapped"
    );
    emu.maps.write_qword(p1, 0xdead_beef_cafe_babe);
    assert_eq!(emu.maps.read_qword(p1).unwrap(), 0xdead_beef_cafe_babe);

    // Second small allocation must not overlap the first.
    let p2 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert!(p2 != 0, "second HeapAlloc returned NULL");
    assert!(p2 != p1, "two allocations returned the same pointer");

    // Large allocation → dedicated map path (>= 0x8000).
    let big = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x20000]);
    assert!(big != 0, "large HeapAlloc returned NULL");
    assert!(emu.maps.is_mapped(big), "large HeapAlloc not mapped");
    emu.maps.write_dword(big + 0x1fff0, 0x11223344);
    assert_eq!(emu.maps.read_dword(big + 0x1fff0).unwrap(), 0x11223344);
}

#[test]
fn heap_realloc_small_to_small_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert!(p1 != 0);
    emu.maps.write_qword(p1, 0xdead_beef_cafe_babe);

    // Grow inside the small/arena path.
    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, 0, p1, 0x400],
    );
    assert!(p2 != 0, "HeapReAlloc returned NULL");
    assert_eq!(
        emu.maps.read_qword(p2).unwrap(),
        0xdead_beef_cafe_babe,
        "content lost during realloc"
    );
}

#[test]
fn heap_realloc_small_to_large_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert!(p1 != 0);
    emu.maps.write_qword(p1, 0x1122_3344_5566_7788);

    // Grow beyond the small/arena threshold to force a dedicated map.
    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, 0, p1, 0x20000],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_qword(p2).unwrap(), 0x1122_3344_5566_7788);
}

#[test]
fn heap_realloc_large_to_large_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x20000]);
    assert!(p1 != 0);
    emu.maps.write_dword(p1 + 0x100, 0xaabbccdd);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, 0, p1, 0x30000],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_dword(p2 + 0x100).unwrap(), 0xaabbccdd);
}

#[test]
fn heap_realloc_shrink_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x20000]);
    assert!(p1 != 0);
    emu.maps.write_qword(p1, 0xfeed_face_dead_beef);
    emu.maps.write_qword(p1 + 0x10000, 0x0011_2233_4455_6677);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, 0, p1, 0x200],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_qword(p2).unwrap(), 0xfeed_face_dead_beef);
}

#[test]
fn heap_realloc_zero_memory_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert!(p1 != 0);
    for i in 0..0x100 {
        emu.maps.write_byte(p1 + i, 0xab);
    }

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, HEAP_ZERO_MEMORY, p1, 0x20000],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_byte(p2).unwrap(), 0xab);
    // The newly-added range must be zeroed.
    for i in 0x100..0x200 {
        assert_eq!(emu.maps.read_byte(p2 + i).unwrap(), 0, "byte at +{:#x}", i);
    }
}

#[test]
fn heap_realloc_invalid_pointer_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    // 0xdead is unmapped.
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn heap_realloc_in_place_shrink_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x20000]);
    assert!(p1 != 0);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, HEAP_REALLOC_IN_PLACE_ONLY, p1, 0x100],
    );
    assert_eq!(p1, p2, "in-place shrink should return the same pointer");
}

#[test]
fn heap_realloc_in_place_grow_fails_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert!(p1 != 0);
    let _blocker =
        helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, HEAP_REALLOC_IN_PLACE_ONLY, p1, 0x20000],
    );
    assert_eq!(ret, 0, "in-place grow should fail");
}

#[test]
fn heap_realloc_zero_size_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, 0, 0x1000, 0],
    );
    assert_eq!(ret, 0);
}

#[test]
fn heap_realloc_null_ptr_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[heap, 0, 0, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn ntdll_rtl_realloc_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    // RtlAllocateHeap always bumps the requested size up to 1024.
    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlAllocateHeap,
        &[heap, 0, 0x100],
    );
    assert!(p1 != 0, "RtlAllocateHeap returned NULL");
    emu.maps.write_qword(p1, 0x9988_7766_5544_3322);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlReAllocateHeap,
        &[heap, 0, p1, 0x400],
    );
    assert!(p2 != 0, "RtlReAllocateHeap returned NULL");
    assert_eq!(emu.maps.read_qword(p2).unwrap(), 0x9988_7766_5544_3322);

    // Invalid pointer must return 0 and not free a real allocation.
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlReAllocateHeap,
        &[heap, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

// GetProcessHeap must return a non-zero handle (the slab slot 0 is
// reserved so the first real handle starts at key 1).
#[test]
fn get_process_heap_returns_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let proc_heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);
    assert_ne!(proc_heap, 0, "GetProcessHeap returned NULL");

    // The returned handle must be a usable heap handle.
    let p = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[proc_heap, 0, 0x100],
    );
    assert_ne!(p, 0, "HeapAlloc via GetProcessHeap returned NULL");
}

// Private-heap lifecycle: create -> alloc -> free -> alloc -> destroy, plus
// zero-size HeapCreate clamping (must not panic).
#[test]
fn heap_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let heap = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapCreate,
        &[0, 0x1000, 0x10000],
    );
    assert_ne!(heap, 0, "HeapCreate returned NULL");

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert_ne!(p1, 0, "HeapAlloc on private heap returned NULL");
    emu.maps.write_dword(p1, 0xfeedface);

    let freed = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapFree, &[heap, 0, p1]);
    assert_eq!(freed, 1, "HeapFree on private heap failed");

    let p2 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert_ne!(p2, 0, "HeapAlloc after free on private heap returned NULL");

    let destroyed = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapDestroy, &[heap]);
    assert_eq!(destroyed, 1, "HeapDestroy failed");
    let again = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapDestroy, &[heap]);
    assert_eq!(again, 0, "second HeapDestroy must fail");

    let zero = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapCreate, &[0, 0, 0]);
    assert_ne!(zero, 0, "HeapCreate(0,0,0) must clamp and succeed");
    let destroyed = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapDestroy, &[zero]);
    assert_eq!(destroyed, 1, "HeapDestroy of clamped heap failed");
}

// Growing a block whose predecessor is free must move it backward in place
// (O1Heap backward expansion) while preserving the payload.
#[test]
fn heap_realloc_backward_move_preserves_data_64() {
    helpers::setup();
    let mut emu = emu64();

    let proc = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);
    assert_ne!(proc, 0);

    let a = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc, 0, 0x8000]);
    let b = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc, 0, 0x100]);
    let _c = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc, 0, 0x100]);
    assert!(a != 0 && b != 0);

    emu.maps.write_dword(b, 0xabcd1234);
    let freed = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapFree, &[proc, 0, a]);
    assert_eq!(freed, 1);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[proc, 0, b, 0x4000],
    );
    assert_eq!(
        p2, a,
        "backward expansion must reuse the freed predecessor address"
    );
    assert_eq!(
        emu.maps.read_dword(p2).unwrap(),
        0xabcd1234,
        "payload must survive the backward move"
    );
}

// Growing a block whose both neighbors are in use must move it (allocate +
// copy + free) while preserving the payload.
#[test]
fn heap_realloc_fallback_move_preserves_data_64() {
    helpers::setup();
    let mut emu = emu64();

    let proc = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);
    assert_ne!(proc, 0);

    let _a = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc, 0, 0x100]);
    let b = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc, 0, 0x100]);
    let _big = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc, 0, 0x8000]);
    assert_ne!(b, 0);

    emu.maps.write_dword(b, 0x55667788);
    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[proc, 0, b, 0x4000],
    );
    assert_ne!(p2, b, "fallback must relocate the block");
    assert_eq!(
        emu.maps.read_dword(p2).unwrap(),
        0x55667788,
        "payload must survive the fallback move"
    );
}

// After a serialize/deserialize roundtrip the runtime heap arenas are gone
// while the `.heap` map survives; the lazy process-heap creation must pick a
// different map name instead of panicking on the collision.
#[test]
fn heap_after_deserialize_64() {
    helpers::setup();

    // Serialization recurses deeply; run the body on a large stack the same
    // way `should_serialize` does.
    let handle = std::thread::Builder::new()
        .stack_size(1024 * 29055)
        .spawn(|| {
            let mut emu = emu64();

            let proc = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);
            assert_ne!(proc, 0);
            let p =
                helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc, 0, 0x100]);
            assert_ne!(p, 0);

            let serialized = Serialization::serialize(&emu);
            let mut emu: Emu = Serialization::deserialize(&serialized);

            let proc2 = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);
            assert_ne!(proc2, 0);
            let p2 =
                helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[proc2, 0, 0x100]);
            assert_ne!(p2, 0, "HeapAlloc after deserialize must succeed");
        })
        .unwrap();

    handle.join().unwrap();
}

#[test]
fn heap_handle_is_real_address_64() {
    helpers::setup();
    let mut emu = emu64();

    let proc = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);
    assert_ne!(proc, 0, "GetProcessHeap must not return NULL");
    assert!(
        proc > 0xFFFF,
        "GetProcessHeap must return a real mapped address, not a bare slab index (got 0x{:x})",
        proc
    );
    assert!(
        emu.maps.read_dword(proc).is_some(),
        "GetProcessHeap address 0x{:x} must be readable (mapped memory)",
        proc
    );

    let proc2 = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);
    assert_eq!(proc, proc2, "GetProcessHeap must be stable across calls");

    let heap = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapCreate,
        &[0, 0x1000, 0x10000],
    );
    assert_ne!(heap, 0, "HeapCreate must not return NULL");
    assert!(
        heap > 0xFFFF,
        "HeapCreate must return a real mapped address, not a bare slab index (got 0x{:x})",
        heap
    );
    assert!(
        emu.maps.read_dword(heap).is_some(),
        "HeapCreate address 0x{:x} must be readable (mapped memory)",
        heap
    );
    assert_ne!(
        heap, proc,
        "HeapCreate must return a different address than the process heap"
    );

    let p = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert_ne!(p, 0, "HeapAlloc with real-address handle must work");

    let destroyed = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapDestroy, &[heap]);
    assert_eq!(
        destroyed, 1,
        "HeapDestroy with real-address handle must work"
    );
}

#[test]
fn global_alloc_free_64() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_ZEROINIT as u64, 0x200],
    );
    assert_ne!(ptr, 0, "GlobalAlloc returned NULL");
    assert!(emu.maps.is_mapped(ptr), "GlobalAlloc pointer not mapped");
    assert_eq!(
        emu.maps.read_byte(ptr).unwrap(),
        0,
        "GMEM_ZEROINIT memory not zeroed"
    );

    let locked = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalLock, &[ptr]);
    assert_eq!(locked, ptr, "GlobalLock must return the pointer itself");

    let unlocked = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalUnlock, &[ptr]);
    assert_eq!(unlocked, 1, "GlobalUnlock must return TRUE");

    let freed = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalFree, &[ptr]);
    assert_eq!(freed, 0, "GlobalFree must return NULL on success");
}

#[test]
fn heap_size_64() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x100]);
    assert_ne!(ptr, 0);

    let sz = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[heap, 0, ptr]);
    assert!(
        sz >= 0x100,
        "HeapSize returned {:#x}, expected >= 0x100",
        sz
    );
    assert_ne!(sz, HEAP_SIZE_ERROR, "HeapSize returned error sentinel");
}

#[test]
fn global_alloc_basic() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_FIXED as u64, 0x200],
    );
    assert_ne!(ptr, 0, "GlobalAlloc returned NULL");
    assert!(emu.maps.is_mapped(ptr), "GlobalAlloc pointer not mapped");

    emu.maps.write_dword(ptr, 0xDEADC0DE);
    assert_eq!(emu.maps.read_dword(ptr).unwrap(), 0xDEADC0DE);
}

#[test]
fn global_alloc_zeroinit() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_ZEROINIT as u64, 0x100],
    );
    assert_ne!(ptr, 0, "GlobalAlloc(GPTR) returned NULL");

    for i in 0..16u64 {
        assert_eq!(
            emu.maps.read_byte(ptr + i).unwrap(),
            0,
            "byte at +{} should be zero after GMEM_ZEROINIT",
            i
        );
    }
}

#[test]
fn global_free() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_FIXED as u64, 0x100],
    );
    assert_ne!(ptr, 0);

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalFree, &[ptr]);
    assert_eq!(ret, 0, "GlobalFree should return NULL on success");
}

#[test]
fn global_lock_unlock() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_FIXED as u64, 0x100],
    );
    assert_ne!(ptr, 0);

    let locked = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalLock, &[ptr]);
    assert_eq!(
        locked, ptr,
        "GlobalLock should return the same pointer for GMEM_FIXED"
    );

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::GlobalUnlock, &[ptr]);
    assert_eq!(ret, 1, "GlobalUnlock returns TRUE");
}

#[test]
fn heap_size() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[heap, 0, 0x200]);
    assert_ne!(ptr, 0);

    let size = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[heap, 0, ptr]);
    assert!(
        size >= 0x200,
        "HeapSize should be >= requested (got {})",
        size
    );
}

#[test]
fn heap_size_invalid_ptr() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let size = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[heap, 0, 0xDEAD]);
    assert_eq!(
        size, HEAP_SIZE_ERROR,
        "HeapSize of invalid ptr should return -1"
    );
}

#[test]
fn global_alloc_heap_size_integration() {
    helpers::setup();
    let mut emu = emu64();
    let heap = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcessHeap, &[]);

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_FIXED as u64, 0x300],
    );
    assert_ne!(ptr, 0);

    let size = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[heap, 0, ptr]);
    assert!(
        size >= 0x300,
        "HeapSize of GlobalAlloc'd memory should be >= requested (got {})",
        size
    );
}

#[test]
fn global_alloc_no_overlap() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_FIXED as u64, 0x200],
    );
    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[GMEM_FIXED as u64, 0x200],
    );
    assert_ne!(p1, 0);
    assert_ne!(p2, 0);
    assert_ne!(
        p1, p2,
        "two GlobalAlloc calls must return different pointers"
    );
}
