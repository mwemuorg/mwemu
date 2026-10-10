use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi32;
use crate::*;

#[test]
fn test_virtual_alloc_32() {
    helpers::setup();
    let mut emu = emu32();

    // VirtualAlloc(lpAddress=0, dwSize=0x1000, MEM_COMMIT|MEM_RESERVE, PAGE_EXECUTE_READWRITE)
    let base = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAlloc,
        &[0, 0x1000, 0x1000 | 0x2000, 0x40],
    );
    assert!(base != 0, "VirtualAlloc 32-bit failed");

    // Verify memory and write
    emu.maps.write_dword(base as u64, 0x11223344);
    let val = emu.maps.read_dword(base as u64).unwrap();
    assert_eq!(val, 0x11223344);
}

#[test]
fn test_write_file_32() {
    helpers::setup();
    let mut emu = emu32();

    // BOOL WriteFile(hFile, lpBuffer, nBytes, lpNumberOfBytesWritten, lpOverlapped)
    let buf_addr = 0x20000u64;
    emu.maps
        .create_map("buffer", buf_addr, 0x2000, Permission::READ_WRITE); // covers written_ptr too
    let written_ptr = 0x21000u64;

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::WriteFile,
        &[0x1234, buf_addr as u32, 100, written_ptr as u32, 0],
    );
    assert_eq!(ret, 1, "WriteFile 32-bit failed");

    // Check bytes written
    let bytes = emu
        .maps
        .read_dword(written_ptr)
        .expect("Cannot read bytes written");
    assert_eq!(bytes, 100);
}

// 32-bit counterpart of the kishou HeapAlloc regression. The 32-bit init path
// (`init_win32_mem32`) never set up `heap_management`, so a small `HeapAlloc`
// panicked on `unwrap()`. `Emu::heap_mut()` now lazily builds the arena.
#[test]
fn test_heap_alloc_32() {
    helpers::setup();
    let mut emu = emu32();

    // HeapAlloc(hHeap, dwFlags, dwBytes) via the stdcall calling convention.
    // Small allocation → managed heap path (< 0x8000).
    let p1 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[0x1234, 0x8, 0x100],
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
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[0x1234, 0, 0x100]) as u64;
    assert!(p2 != 0, "second HeapAlloc returned NULL");
    assert!(p2 != p1, "two allocations returned the same pointer");

    // Large allocation → dedicated map path (>= 0x8000).
    let big = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[0x1234, 0, 0x20000],
    ) as u64;
    assert!(big != 0, "large HeapAlloc returned NULL");
    assert!(emu.maps.is_mapped(big), "large HeapAlloc not mapped");
}

#[test]
fn test_heap_realloc_small_to_small_32() {
    helpers::setup();
    let mut emu = emu32();

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[0x1234, 0, 0x100]) as u64;
    assert!(p1 != 0);
    emu.maps.write_dword(p1, 0xdeadbeef);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0, p1 as u32, 0x400],
    ) as u64;
    assert!(p2 != 0, "HeapReAlloc returned NULL");
    assert_eq!(emu.maps.read_dword(p2).unwrap(), 0xdeadbeef);
}

#[test]
fn test_heap_realloc_small_to_large_32() {
    helpers::setup();
    let mut emu = emu32();

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[0x1234, 0, 0x100]) as u64;
    assert!(p1 != 0);
    emu.maps.write_dword(p1, 0x11223344);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0, p1 as u32, 0x20000],
    ) as u64;
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_dword(p2).unwrap(), 0x11223344);
}

#[test]
fn test_heap_realloc_large_to_large_32() {
    helpers::setup();
    let mut emu = emu32();

    let p1 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[0x1234, 0, 0x20000],
    ) as u64;
    assert!(p1 != 0);
    emu.maps.write_dword(p1 + 0x100, 0xaabbccdd);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0, p1 as u32, 0x30000],
    ) as u64;
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_dword(p2 + 0x100).unwrap(), 0xaabbccdd);
}

#[test]
fn test_heap_realloc_zero_memory_32() {
    helpers::setup();
    let mut emu = emu32();

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[0x1234, 0, 0x100]) as u64;
    assert!(p1 != 0);
    for i in 0..0x100 {
        emu.maps.write_byte(p1 + i, 0xab);
    }

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0x8, p1 as u32, 0x20000],
    ) as u64;
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_byte(p2).unwrap(), 0xab);
    for i in 0x100..0x200 {
        assert_eq!(emu.maps.read_byte(p2 + i).unwrap(), 0, "byte at +{:#x}", i);
    }
}

#[test]
fn test_heap_realloc_invalid_pointer_32() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn test_heap_realloc_in_place_shrink_32() {
    helpers::setup();
    let mut emu = emu32();

    let p1 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapAlloc,
        &[0x1234, 0, 0x20000],
    ) as u64;
    assert!(p1 != 0);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0x10, p1 as u32, 0x100],
    ) as u64;
    assert_eq!(p1, p2);
}

#[test]
fn test_heap_realloc_in_place_grow_fails_32() {
    helpers::setup();
    let mut emu = emu32();

    let p1 =
        helpers::call_winapi32(&mut emu, winapi32::kernel32::HeapAlloc, &[0x1234, 0, 0x100]) as u64;
    assert!(p1 != 0);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0x10, p1 as u32, 0x20000],
    );
    assert_eq!(ret, 0);
}

#[test]
fn test_heap_realloc_zero_size_32() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0, 0x1000, 0],
    );
    assert_eq!(ret, 0);
}

#[test]
fn test_heap_realloc_null_ptr_32() {
    helpers::setup();
    let mut emu = emu32();

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::HeapReAlloc,
        &[0x1234, 0, 0, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn test_ntdll_rtl_realloc_32() {
    helpers::setup();
    let mut emu = emu32();

    let p1 = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlAllocateHeap,
        &[0x1234, 0, 0x100],
    ) as u64;
    assert!(p1 != 0, "RtlAllocateHeap returned NULL");
    emu.maps.write_dword(p1, 0x99887766);

    let p2 = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlReAllocateHeap,
        &[0x1234, 0, p1 as u32, 0x400],
    ) as u64;
    assert!(p2 != 0, "RtlReAllocateHeap returned NULL");
    assert_eq!(emu.maps.read_dword(p2).unwrap(), 0x99887766);

    let ret = helpers::call_winapi32(
        &mut emu,
        winapi32::ntdll::RtlReAllocateHeap,
        &[0x1234, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

// Regression: the export-index refactor inverted the IS_INTRESOURCE test, so
// every name pointer (>= 0x10000) was classified as an ordinal and by-name
// GetProcAddress always returned NULL. lpProcName is an ordinal only when its
// high word is zero.
#[test]
fn test_get_proc_address_by_name_and_ordinal_32() {
    helpers::setup();
    let mut emu = emu32();

    let base = 0x70100000u64;
    helpers::register_fake_export_module(&mut emu, base);

    let name_ptr = 0x30000u64;
    emu.maps
        .create_map("gpa_name", name_ptr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_ptr, "CreateFileA");

    let by_name = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::GetProcAddress,
        &[base as u32, name_ptr as u32],
    ) as u64;
    assert_eq!(by_name, base + 0x1500, "by-name lookup returned NULL");

    // export_base is 5, so ordinal 5 maps to function-table slot 0.
    let by_ordinal = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::GetProcAddress,
        &[base as u32, 5],
    ) as u64;
    assert_eq!(by_ordinal, base + 0x1500, "by-ordinal lookup returned NULL");
}

// The exact IS_INTRESOURCE boundary: 0xFFFF is the highest possible ordinal,
// 0x10000 is the lowest possible name pointer. With the inverted predicate,
// 0x10000 was masked to ordinal 0 and returned NULL.
#[test]
fn test_get_proc_address_intresource_boundary_32() {
    helpers::setup();
    let mut emu = emu32();

    let base = 0x70100000u64;
    helpers::register_fake_export_module(&mut emu, base);

    // A name string mapped exactly at 0x10000 must go through the name path.
    let name_ptr = 0x10000u64;
    emu.maps
        .create_map("gpa_boundary", name_ptr, 0x1000, Permission::READ_WRITE)
        .expect("cannot map 0x10000");
    emu.maps.write_string(name_ptr, "CreateFileA");

    let by_name = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::GetProcAddress,
        &[base as u32, name_ptr as u32],
    ) as u64;
    assert_eq!(
        by_name,
        base + 0x1500,
        "0x10000 must be treated as a name pointer"
    );

    // 0xFFFF is an ordinal (unknown here) — must return 0, and must never be
    // dereferenced as a string pointer (0xFFFF is unmapped).
    let by_ordinal = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::GetProcAddress,
        &[base as u32, 0xFFFF],
    );
    assert_eq!(by_ordinal, 0, "unknown ordinal must resolve to NULL");
}

// The export-index registry lookup is case-insensitive, matching the
// pre-index PEB-scanner behavior the emulator has always had.
#[test]
fn test_get_proc_address_case_insensitive_32() {
    helpers::setup();
    let mut emu = emu32();

    let base = 0x70100000u64;
    helpers::register_fake_export_module(&mut emu, base);

    let name_ptr = 0x30000u64;
    emu.maps
        .create_map("gpa_name", name_ptr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_ptr, "CREATEfileA");

    let addr = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::GetProcAddress,
        &[base as u32, name_ptr as u32],
    ) as u64;
    assert_eq!(addr, base + 0x1500, "lookup must be case-insensitive");
}

// A registered handle whose module does not export the requested name must
// return NULL honoring the handle, not silently fall back to a global search.
#[test]
fn test_get_proc_address_missing_export_honors_handle_32() {
    helpers::setup();
    let mut emu = emu32();

    let base = 0x70100000u64;
    helpers::register_fake_export_module(&mut emu, base);

    let name_ptr = 0x30000u64;
    emu.maps
        .create_map("gpa_name", name_ptr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_ptr, "NotExported");

    let addr = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::GetProcAddress,
        &[base as u32, name_ptr as u32],
    );
    assert_eq!(
        addr, 0,
        "missing export must return NULL for a known handle"
    );
}

// GetProcessHeap must return a non-zero handle (the slab slot 0 is
// reserved so the first real handle starts at key 1).
#[test]
fn test_get_process_heap_returns_handle_32() {
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
fn test_heap_lifecycle_32() {
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
fn test_heap_realloc_backward_move_preserves_data_32() {
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
fn test_heap_realloc_fallback_move_preserves_data_32() {
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
fn test_heap_handle_is_real_address_32() {
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
