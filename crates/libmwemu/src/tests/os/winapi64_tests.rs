use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::*; // Assuming crate root has winapi module public or we can access it.
// If `winapi` mod is not public, we might have issues.
// Existing tests import `use crate::*;`.
// `lib.rs` usually has `pub mod winapi;`.

#[test]
fn test_write_file() {
    helpers::setup();
    let mut emu = emu64();

    // Setup buffer
    let buff_addr = 0x100000;
    emu.maps
        .create_map("buffer", buff_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(buff_addr, "Hello WinAPI");

    let written_ptr = 0x200000;
    emu.maps
        .create_map("written", written_ptr, 0x1000, Permission::READ_WRITE);

    // BOOL WriteFile(hFile, lpBuffer, nBytes, lpNumberOfBytesWritten, lpOverlapped)
    // The 5th argument (lpOverlapped = NULL) is passed on the stack at rsp+0x20.
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::WriteFile,
        &[0x1234, buff_addr, 12, written_ptr, 0],
    );

    // RAX should be 1 (TRUE)
    assert_eq!(ret, 1, "WriteFile failed (returned 0)");

    // Read bytes written
    let bytes = emu.maps.read_dword(written_ptr).unwrap();
    assert_eq!(bytes, 12);
}

#[test]
fn test_get_module_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    // HMODULE GetModuleHandleA(
    //   LPCSTR lpModuleName
    // );

    // "kernel32.dll"
    let name_addr = 0x20000;
    emu.maps
        .create_map("data", name_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_addr, "kernel32.dll");

    // Create the expected module map "kernel32.pe"
    emu.maps.create_map(
        "kernel32.pe",
        0x7FF10000000,
        0x10000,
        Permission::READ_EXECUTE,
    );

    let h_mod =
        helpers::call_winapi64(&mut emu, winapi64::kernel32::GetModuleHandleA, &[name_addr]);
    assert_eq!(
        h_mod, 0x7FF10000000,
        "GetModuleHandleA('kernel32.dll') returned incorrect base"
    );
}

#[test]
fn test_close_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    // CloseHandle checks if handle exists in global map.
    // If not, it panics.
    // We need to create a valid handle first.
    // Use `handler_create` from helper? It's pub?
    // helper::handler_create(name) -> handle

    let handle = crate::winapi::helper::handler_create("dummy_file");

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::CloseHandle, &[handle]);

    // Expect 1
    assert_eq!(ret, 1);
}

#[test]
fn test_virtual_alloc() {
    helpers::setup();
    let mut emu = emu64();

    // LPVOID VirtualAlloc(
    //   LPVOID lpAddress,
    //   SIZE_T dwSize,
    //   DWORD  flAllocationType,
    //   DWORD  flProtect
    // );

    // VirtualAlloc(lpAddress=0, dwSize=0x1000, MEM_COMMIT|MEM_RESERVE, PAGE_EXECUTE_READWRITE)
    let base = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[0, 0x1000, 0x1000 | 0x2000, 0x40],
    );
    assert!(base != 0, "VirtualAlloc failed");

    // Verify memory access
    emu.maps.write_dword(base, 0xDEADBEEF);
    let val = emu.maps.read_dword(base).unwrap();
    assert_eq!(val, 0xDEADBEEF);
}

// Regression test for the heap bug reported by kishou: a small `HeapAlloc`
// panicked because `heap_management` was `None` unless the 64-bit normal-mode
// init had run (it is left `None` by the 32-bit path, by SSDT/syscall mode,
// and right after deserialization). `Emu::heap_mut()` now creates the arena
// lazily, so a bare `emu64()` can allocate without any prior init.
#[test]
fn test_heap_alloc_64() {
    helpers::setup();
    let mut emu = emu64();

    // HeapAlloc(hHeap, dwFlags, dwBytes) via the x64 calling convention.
    // Small allocation → managed heap path (< 0x8000).
    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0x8, 0x100],
    );
    assert!(p1 != 0, "HeapAlloc(0x100) returned NULL");
    assert!(
        emu.maps.is_mapped(p1),
        "HeapAlloc(0x100) pointer not mapped"
    );
    emu.maps.write_qword(p1, 0xdead_beef_cafe_babe);
    assert_eq!(emu.maps.read_qword(p1).unwrap(), 0xdead_beef_cafe_babe);

    // Second small allocation must not overlap the first.
    let p2 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[0x1234, 0, 0x100]);
    assert!(p2 != 0, "second HeapAlloc returned NULL");
    assert!(p2 != p1, "two allocations returned the same pointer");

    // Large allocation → dedicated map path (>= 0x8000).
    let big = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x20000],
    );
    assert!(big != 0, "large HeapAlloc returned NULL");
    assert!(emu.maps.is_mapped(big), "large HeapAlloc not mapped");
    emu.maps.write_dword(big + 0x1fff0, 0x11223344);
    assert_eq!(emu.maps.read_dword(big + 0x1fff0).unwrap(), 0x11223344);
}

#[test]
fn test_heap_realloc_small_to_small_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[0x1234, 0, 0x100]);
    assert!(p1 != 0);
    emu.maps.write_qword(p1, 0xdead_beef_cafe_babe);

    // Grow inside the small/arena path.
    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, p1, 0x400],
    );
    assert!(p2 != 0, "HeapReAlloc returned NULL");
    assert_eq!(
        emu.maps.read_qword(p2).unwrap(),
        0xdead_beef_cafe_babe,
        "content lost during realloc"
    );
}

#[test]
fn test_heap_realloc_small_to_large_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[0x1234, 0, 0x100]);
    assert!(p1 != 0);
    emu.maps.write_qword(p1, 0x1122_3344_5566_7788);

    // Grow beyond the small/arena threshold to force a dedicated map.
    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, p1, 0x20000],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_qword(p2).unwrap(), 0x1122_3344_5566_7788);
}

#[test]
fn test_heap_realloc_large_to_large_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x20000],
    );
    assert!(p1 != 0);
    emu.maps.write_dword(p1 + 0x100, 0xaabbccdd);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, p1, 0x30000],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_dword(p2 + 0x100).unwrap(), 0xaabbccdd);
}

#[test]
fn test_heap_realloc_shrink_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x20000],
    );
    assert!(p1 != 0);
    emu.maps.write_qword(p1, 0xfeed_face_dead_beef);
    emu.maps.write_qword(p1 + 0x10000, 0x0011_2233_4455_6677);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, p1, 0x200],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_qword(p2).unwrap(), 0xfeed_face_dead_beef);
}

#[test]
fn test_heap_realloc_zero_memory_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[0x1234, 0, 0x100]);
    assert!(p1 != 0);
    for i in 0..0x100 {
        emu.maps.write_byte(p1 + i, 0xab);
    }

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0x8, p1, 0x20000],
    );
    assert!(p2 != 0);
    assert_eq!(emu.maps.read_byte(p2).unwrap(), 0xab);
    // The newly-added range must be zeroed.
    for i in 0x100..0x200 {
        assert_eq!(emu.maps.read_byte(p2 + i).unwrap(), 0, "byte at +{:#x}", i);
    }
}

#[test]
fn test_heap_realloc_invalid_pointer_64() {
    helpers::setup();
    let mut emu = emu64();

    // 0xdead is unmapped.
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn test_heap_realloc_in_place_shrink_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x20000],
    );
    assert!(p1 != 0);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0x10, p1, 0x100],
    );
    assert_eq!(p1, p2, "in-place shrink should return the same pointer");
}

#[test]
fn test_heap_realloc_in_place_grow_fails_64() {
    helpers::setup();
    let mut emu = emu64();

    let p1 = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[0x1234, 0, 0x100]);
    assert!(p1 != 0);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0x10, p1, 0x20000],
    );
    assert_eq!(ret, 0, "in-place grow should fail");
}

#[test]
fn test_heap_realloc_zero_size_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, 0x1000, 0],
    );
    assert_eq!(ret, 0);
}

#[test]
fn test_heap_realloc_null_ptr_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, 0, 0x100],
    );
    assert_eq!(ret, 0);
}

#[test]
fn test_ntdll_rtl_realloc_64() {
    helpers::setup();
    let mut emu = emu64();

    // RtlAllocateHeap always bumps the requested size up to 1024.
    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlAllocateHeap,
        &[0x1234, 0, 0x100],
    );
    assert!(p1 != 0, "RtlAllocateHeap returned NULL");
    emu.maps.write_qword(p1, 0x9988_7766_5544_3322);

    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlReAllocateHeap,
        &[0x1234, 0, p1, 0x400],
    );
    assert!(p2 != 0, "RtlReAllocateHeap returned NULL");
    assert_eq!(emu.maps.read_qword(p2).unwrap(), 0x9988_7766_5544_3322);

    // Invalid pointer must return 0 and not free a real allocation.
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlReAllocateHeap,
        &[0x1234, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}

// GetProcessHeap must return a non-zero handle (the slab slot 0 is
// reserved so the first real handle starts at key 1).
#[test]
fn test_get_process_heap_returns_handle_64() {
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
// Regression: the 64-bit ordinal mask (0xFFFF_0000_0000_0000) was never set by
// real arguments, so the ordinal path was unreachable and ordinal calls did
// read_string(ordinal) -> NULL. lpProcName is an ordinal only when its high
// word is zero.
#[test]
fn test_get_proc_address_by_name_and_ordinal_64() {
    helpers::setup();
    let mut emu = emu64();

    let base = 0x7ff0_0010_0000u64;
    helpers::register_fake_export_module(&mut emu, base);

    let name_ptr = 0x300000u64;
    emu.maps
        .create_map("gpa_name", name_ptr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_ptr, "CreateFileA");

    let by_name = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GetProcAddress,
        &[base, name_ptr],
    );
    assert_eq!(by_name, base + 0x1500, "by-name lookup returned NULL");

    // export_base is 5, so ordinal 5 maps to function-table slot 0.
    let by_ordinal =
        helpers::call_winapi64(&mut emu, winapi64::kernel32::GetProcAddress, &[base, 5]);
    assert_eq!(by_ordinal, base + 0x1500, "by-ordinal lookup returned NULL");
}

// The exact IS_INTRESOURCE boundary on 64 bits: 0xFFFF is the highest
// possible ordinal, 0x10000 the lowest possible name pointer.
#[test]
fn test_get_proc_address_intresource_boundary_64() {
    helpers::setup();
    let mut emu = emu64();

    let base = 0x7ff0_0010_0000u64;
    helpers::register_fake_export_module(&mut emu, base);

    let name_ptr = 0x10000u64;
    emu.maps
        .create_map("gpa_boundary", name_ptr, 0x1000, Permission::READ_WRITE)
        .expect("cannot map 0x10000");
    emu.maps.write_string(name_ptr, "CreateFileA");

    let by_name = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GetProcAddress,
        &[base, name_ptr],
    );
    assert_eq!(
        by_name,
        base + 0x1500,
        "0x10000 must be treated as a name pointer"
    );

    let by_ordinal = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GetProcAddress,
        &[base, 0xFFFF],
    );
    assert_eq!(by_ordinal, 0, "unknown ordinal must resolve to NULL");
}

// GetProcAddress must chase export forwarders (`OTHER.Symbol`) through the
// registry, both by name and by ordinal.
#[test]
fn test_get_proc_address_forwarder_64() {
    use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

    helpers::setup();
    let mut emu = emu64();

    let fake_base = 0x7ff0_0010_0000u64;
    let backing_base = 0x7ff0_0200_0000u64;

    // fake.dll exports "ViaFwd" (ordinal 1) forwarding to backing.TargetFn.
    let fake = ExportIndexData {
        export_base: 1,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Forwarder {
            value: "backing.TargetFn".to_string(),
        })],
        named_exports: vec![NamedExport {
            name: "ViaFwd".to_string(),
            ordinal_index: 0,
        }],
    };
    // backing.dll exports "TargetFn" at backing_base + 0x2000.
    let backing = ExportIndexData {
        export_base: 1,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Direct { rva: 0x2000 })],
        named_exports: vec![NamedExport {
            name: "TargetFn".to_string(),
            ordinal_index: 0,
        }],
    };
    helpers::register_export_module(&mut emu, "fake.dll", fake_base, &fake);
    helpers::register_export_module(&mut emu, "backing.dll", backing_base, &backing);

    let name_ptr = 0x300000u64;
    emu.maps
        .create_map("gpa_name", name_ptr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_ptr, "ViaFwd");

    let by_name = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GetProcAddress,
        &[fake_base, name_ptr],
    );
    assert_eq!(
        by_name,
        backing_base + 0x2000,
        "forwarder by name must resolve into backing.dll"
    );

    let by_ordinal = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GetProcAddress,
        &[fake_base, 1],
    );
    assert_eq!(
        by_ordinal,
        backing_base + 0x2000,
        "forwarder by ordinal must resolve into backing.dll"
    );
}

// Private-heap lifecycle: create -> alloc -> free -> alloc -> destroy, plus
// zero-size HeapCreate clamping (must not panic).
#[test]
fn test_heap_lifecycle_64() {
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
fn test_heap_realloc_backward_move_preserves_data_64() {
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
fn test_heap_realloc_fallback_move_preserves_data_64() {
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
fn test_heap_after_deserialize_64() {
    use crate::serialization::Serialization;

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
fn test_heap_handle_is_real_address_64() {
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

// ── kernel32: GlobalAlloc / GlobalFree / GlobalLock / GlobalUnlock ──

#[test]
fn test_global_alloc_free_64() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::GlobalAlloc,
        &[0x0040, 0x200], // GPTR (GMEM_FIXED|GMEM_ZEROINIT)
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

// ── kernel32: HeapSize ──

#[test]
fn test_heap_size_64() {
    helpers::setup();
    let mut emu = emu64();

    let ptr = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapAlloc, &[0x1234, 0, 0x100]);
    assert_ne!(ptr, 0);

    let sz = helpers::call_winapi64(&mut emu, winapi64::kernel32::HeapSize, &[0x1234, 0, ptr]);
    assert!(
        sz >= 0x100,
        "HeapSize returned {:#x}, expected >= 0x100",
        sz
    );
    assert_ne!(sz, 0xFFFFFFFFFFFFFFFF, "HeapSize returned error sentinel");
}

// ── kernel32: SRWLock family ──

#[test]
fn test_srw_lock_roundtrip_64() {
    helpers::setup();
    let mut emu = emu64();

    let lock_addr = 0x100000u64;
    emu.maps
        .create_map("srw_lock", lock_addr, 0x1000, Permission::READ_WRITE);

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::InitializeSRWLock,
        &[lock_addr],
    );
    assert_eq!(
        emu.maps.read_qword(lock_addr).unwrap(),
        0,
        "InitializeSRWLock must zero the lock"
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::AcquireSRWLockExclusive,
        &[lock_addr],
    );
    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::ReleaseSRWLockExclusive,
        &[lock_addr],
    );

    let acquired = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockExclusive,
        &[lock_addr],
    );
    assert_eq!(acquired, 1, "TryAcquireSRWLockExclusive must succeed");

    let shared = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockShared,
        &[lock_addr],
    );
    assert_eq!(shared, 1, "TryAcquireSRWLockShared must succeed");
}

// ── kernel32: WaitForMultipleObjects ──

#[test]
fn test_wait_for_multiple_objects_64() {
    helpers::setup();
    let mut emu = emu64();

    let handles_addr = 0x100000u64;
    emu.maps
        .create_map("handles", handles_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(handles_addr, 0x100);
    emu.maps.write_qword(handles_addr + 8, 0x200);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::WaitForMultipleObjects,
        &[2, handles_addr, 0, 0],
    );
    assert_eq!(ret, 0, "WaitForMultipleObjects must return WAIT_OBJECT_0");
}

// ── kernel32: GetTickCount64 / QueryPerformanceFrequency ──

#[test]
fn test_get_tick_count64() {
    helpers::setup();
    let mut emu = emu64();

    let tick = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount64, &[]);
    assert!(tick < u64::MAX, "GetTickCount64 returned max u64");
}

#[test]
fn test_query_performance_frequency_64() {
    helpers::setup();
    let mut emu = emu64();

    let freq_addr = 0x100000u64;
    emu.maps
        .create_map("freq", freq_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::QueryPerformanceFrequency,
        &[freq_addr],
    );
    assert_eq!(ret, 1, "QueryPerformanceFrequency must return TRUE");
    let freq = emu.maps.read_qword(freq_addr).unwrap();
    assert!(freq > 0, "frequency must be non-zero");
}

// ── kernel32: IsWow64Process ──

#[test]
fn test_is_wow64_process_64() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("wow64", out_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(out_addr, 0xDEAD);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::IsWow64Process,
        &[0xFFFFFFFFFFFFFFFF, out_addr], // current process pseudo-handle
    );
    assert_eq!(ret, 1, "IsWow64Process must return TRUE");
    let wow64 = emu.maps.read_qword(out_addr).unwrap();
    assert_eq!(wow64, 0, "64-bit process must not be WoW64");
}

// ── kernel32: DuplicateHandle ──

#[test]
fn test_duplicate_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("duphandle", out_addr, 0x1000, Permission::READ_WRITE);

    let src_handle = 0x42u64;
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::DuplicateHandle,
        &[
            0xFFFFFFFFFFFFFFFF, // hSourceProcess (current)
            src_handle,         // hSourceHandle
            0xFFFFFFFFFFFFFFFF, // hTargetProcess (current)
            out_addr,           // lpTargetHandle
            0,                  // dwDesiredAccess
            0,                  // bInheritHandle
            0,                  // dwOptions
        ],
    );
    assert_eq!(ret, 1, "DuplicateHandle must return TRUE");
    let dup = emu.maps.read_qword(out_addr).unwrap();
    assert_eq!(dup, src_handle, "duplicated handle must equal source");
}

// ── kernel32: CreateDirectoryA/W ──

#[test]
fn test_create_directory_a_64() {
    helpers::setup();
    let mut emu = emu64();

    let path_addr = 0x100000u64;
    emu.maps
        .create_map("dirname", path_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(path_addr, "C:\\Temp\\TestDir");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::CreateDirectoryA,
        &[path_addr, 0],
    );
    assert_eq!(ret, 1, "CreateDirectoryA must return TRUE");
}

// ── kernel32: FreeLibrary ──

#[test]
fn test_free_library_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::FreeLibrary, &[0x7FF00000]);
    assert_eq!(ret, 1, "FreeLibrary must return TRUE");
}

// ── kernel32: SetEnvironmentVariableA ──

#[test]
fn test_set_environment_variable_a_64() {
    helpers::setup();
    let mut emu = emu64();

    let name_addr = 0x100000u64;
    let val_addr = 0x100100u64;
    emu.maps
        .create_map("envdata", name_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_addr, "MY_VAR");
    emu.maps.write_string(val_addr, "hello_world");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::SetEnvironmentVariableA,
        &[name_addr, val_addr],
    );
    assert_eq!(ret, 1, "SetEnvironmentVariableA must return TRUE");
}

// ── ntdll: NtCreateSection + NtMapViewOfSection + NtUnmapViewOfSection ──

#[test]
fn test_section_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let handle_ptr = 0x100000u64;
    let base_ptr = 0x100010u64;
    let view_sz_ptr = 0x100020u64;
    let max_sz_ptr = 0x100030u64;
    emu.maps
        .create_map("section_io", 0x100000, 0x1000, Permission::READ_WRITE);

    emu.maps.write_qword(max_sz_ptr, 0x10000);
    emu.maps.write_qword(view_sz_ptr, 0x10000);
    emu.maps.write_qword(base_ptr, 0);

    // NtCreateSection(SectionHandle*, Access, ObjAttr, MaxSize*, Prot, Alloc, File)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtCreateSection,
        &[handle_ptr, 0xF001F, 0, max_sz_ptr, 0x04, 0x8000000, 0],
    );
    assert_eq!(status, 0, "NtCreateSection must return STATUS_SUCCESS");
    let section_handle = emu.maps.read_qword(handle_ptr).unwrap();
    assert_ne!(section_handle, 0, "section handle must be non-zero");

    // NtMapViewOfSection(Section, Process, BaseAddr*, ZeroBits, CommitSz, Offset, ViewSz*, Inherit, AllocType, Protect)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtMapViewOfSection,
        &[
            section_handle,
            0xFFFFFFFFFFFFFFFF,
            base_ptr,
            0,
            0,
            0,
            view_sz_ptr,
            1,
            0,
            0x04,
        ],
    );
    assert_eq!(status, 0, "NtMapViewOfSection must return STATUS_SUCCESS");
    let base = emu.maps.read_qword(base_ptr).unwrap();
    assert_ne!(base, 0, "mapped base must be non-zero");
    assert!(emu.maps.is_mapped(base), "mapped memory must be accessible");

    emu.maps.write_dword(base, 0xCAFEBABE);
    assert_eq!(emu.maps.read_dword(base).unwrap(), 0xCAFEBABE);

    // NtUnmapViewOfSection(Process, BaseAddress)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtUnmapViewOfSection,
        &[0xFFFFFFFFFFFFFFFF, base],
    );
    assert_eq!(status, 0, "NtUnmapViewOfSection must return STATUS_SUCCESS");
}

// ── ntdll: NtFreeVirtualMemory ──

#[test]
fn test_nt_free_virtual_memory_64() {
    helpers::setup();
    let mut emu = emu64();

    let base = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[0, 0x1000, 0x3000, 0x40],
    );
    assert_ne!(base, 0);
    assert!(emu.maps.is_mapped(base));

    let base_ptr = 0x200000u64;
    emu.maps
        .create_map("ntfree_io", base_ptr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(base_ptr, base);

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtFreeVirtualMemory,
        &[0xFFFFFFFFFFFFFFFF, base_ptr, 0, 0x8000],
    );
    assert_eq!(status, 0, "NtFreeVirtualMemory must return STATUS_SUCCESS");
}

// ── ntdll: RtlInitUnicodeString ──

#[test]
fn test_rtl_init_unicode_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let ustr_addr = 0x100000u64;
    let src_addr = 0x100100u64;
    emu.maps
        .create_map("ustr_io", 0x100000, 0x1000, Permission::READ_WRITE);

    // Write L"Hello" (wide) — 5 chars + null = 12 bytes
    let hello: [u8; 12] = [b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, 0, 0];
    for (i, &b) in hello.iter().enumerate() {
        emu.maps.write_byte(src_addr + i as u64, b);
    }

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitUnicodeString,
        &[ustr_addr, src_addr],
    );

    let length = emu.maps.read_word(ustr_addr).unwrap();
    let max_length = emu.maps.read_word(ustr_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(ustr_addr + 8).unwrap();
    assert_eq!(length, 10, "Length must be 5 chars * 2 bytes = 10");
    assert_eq!(max_length, 12, "MaximumLength must be Length + 2");
    assert_eq!(buffer, src_addr, "Buffer must point to the source string");
}

// ── ntdll: RtlInitAnsiString ──

#[test]
fn test_rtl_init_ansi_string_64() {
    helpers::setup();
    let mut emu = emu64();

    let astr_addr = 0x100000u64;
    let src_addr = 0x100100u64;
    emu.maps
        .create_map("astr_io", 0x100000, 0x1000, Permission::READ_WRITE);

    emu.maps.write_string(src_addr, "Test");

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlInitAnsiString,
        &[astr_addr, src_addr],
    );

    let length = emu.maps.read_word(astr_addr).unwrap();
    let max_length = emu.maps.read_word(astr_addr + 2).unwrap();
    let buffer = emu.maps.read_qword(astr_addr + 8).unwrap();
    assert_eq!(length, 4, "Length must be 4 (strlen)");
    assert_eq!(max_length, 5, "MaximumLength must be Length + 1");
    assert_eq!(buffer, src_addr, "Buffer must point to the source string");
}

// ── ntdll: NtQueryInformationProcess ──

#[test]
fn test_nt_query_information_process_64() {
    helpers::setup();
    let mut emu = emu64();

    let info_addr = 0x100000u64;
    let ret_len_addr = 0x100100u64;
    emu.maps
        .create_map("nqip_io", 0x100000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(info_addr, 0xDEAD);

    // Class 0 = ProcessBasicInformation
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtQueryInformationProcess,
        &[0xFFFFFFFFFFFFFFFF, 0, info_addr, 48, ret_len_addr],
    );
    assert_eq!(
        status, 0,
        "NtQueryInformationProcess must return STATUS_SUCCESS"
    );
}

// ── advapi32: CryptAcquireContextA + CryptGenRandom + CryptReleaseContext ──

#[test]
fn test_crypt_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let prov_addr = 0x100000u64;
    let buf_addr = 0x100100u64;
    emu.maps
        .create_map("crypt_io", 0x100000, 0x1000, Permission::READ_WRITE);

    let container_addr = 0x200000u64;
    let provider_addr = 0x200100u64;
    emu.maps
        .create_map("crypt_str", 0x200000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(container_addr, "TestContainer");
    emu.maps
        .write_string(provider_addr, "Microsoft Base Cryptographic Provider v1.0");

    // CryptAcquireContextA(phProv, pszContainer, pszProvider, dwProvType, dwFlags)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptAcquireContextA,
        &[prov_addr, container_addr, provider_addr, 1, 0],
    );
    assert_eq!(ret, 1, "CryptAcquireContextA must return TRUE");
    let hprov = emu.maps.read_qword(prov_addr).unwrap();
    assert_ne!(hprov, 0, "hProv must be non-zero");

    // CryptGenRandom(hProv, dwLen, pbBuffer)
    for i in 0..16u64 {
        emu.maps.write_byte(buf_addr + i, 0);
    }
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptGenRandom,
        &[hprov, 16, buf_addr],
    );
    assert_eq!(ret, 1, "CryptGenRandom must return TRUE");

    // CryptReleaseContext(hProv, dwFlags)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptReleaseContext,
        &[hprov, 0],
    );
    assert_eq!(ret, 1, "CryptReleaseContext must return TRUE");
}

// ── advapi32: CryptCreateHash + CryptHashData ──

#[test]
fn test_crypt_hash_64() {
    helpers::setup();
    let mut emu = emu64();

    let prov_addr = 0x100000u64;
    let hash_addr = 0x100010u64;
    let data_addr = 0x100100u64;
    emu.maps
        .create_map("hash_io", 0x100000, 0x1000, Permission::READ_WRITE);

    let container_addr = 0x200000u64;
    emu.maps
        .create_map("hash_str", 0x200000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(container_addr, "test");

    // Acquire context first
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptAcquireContextA,
        &[prov_addr, container_addr, 0, 1, 0],
    );
    assert_eq!(ret, 1);
    let hprov = emu.maps.read_qword(prov_addr).unwrap();

    // CryptCreateHash(hProv, Algid=MD5=0x8003, hKey=0, dwFlags=0, phHash)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptCreateHash,
        &[hprov, 0x8003, 0, 0, hash_addr],
    );
    assert_eq!(ret, 1, "CryptCreateHash must return TRUE");
    let hhash = emu.maps.read_qword(hash_addr).unwrap();
    assert_ne!(hhash, 0, "hHash must be non-zero");

    // CryptHashData(hHash, pbData, dwDataLen, dwFlags)
    emu.maps.write_string(data_addr, "hello world");
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::CryptHashData,
        &[hhash, data_addr, 11, 0],
    );
    assert_eq!(ret, 1, "CryptHashData must return TRUE");
}

// ── advapi32: AdjustTokenPrivileges ──

#[test]
fn test_adjust_token_privileges_64() {
    helpers::setup();
    let mut emu = emu64();

    // AdjustTokenPrivileges(TokenHandle, DisableAll, NewState, BufLen, PrevState, RetLen)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::AdjustTokenPrivileges,
        &[0x1234, 0, 0, 0, 0, 0],
    );
    assert_eq!(ret, 1, "AdjustTokenPrivileges must return TRUE");
}

// ── user32: MessageBoxW ──

#[test]
fn test_message_box_w_64() {
    helpers::setup();
    let mut emu = emu64();

    let text_addr = 0x100000u64;
    let caption_addr = 0x100100u64;
    emu.maps
        .create_map("msgbox_io", 0x100000, 0x1000, Permission::READ_WRITE);

    // Write L"Alert" as caption
    let caption: [u8; 12] = [b'A', 0, b'l', 0, b'e', 0, b'r', 0, b't', 0, 0, 0];
    for (i, &b) in caption.iter().enumerate() {
        emu.maps.write_byte(caption_addr + i as u64, b);
    }
    // Write L"OK" as text
    let text: [u8; 6] = [b'O', 0, b'K', 0, 0, 0];
    for (i, &b) in text.iter().enumerate() {
        emu.maps.write_byte(text_addr + i as u64, b);
    }

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::user32::MessageBoxW,
        &[0, text_addr, caption_addr, 0],
    );
    assert_eq!(ret, 1, "MessageBoxW must return IDOK (1)");
}

// ── user32: FindWindowA ──

#[test]
fn test_find_window_a_64() {
    helpers::setup();
    let mut emu = emu64();

    let class_addr = 0x100000u64;
    emu.maps
        .create_map("fw_io", 0x100000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(class_addr, "Notepad");

    let ret = helpers::call_winapi64(&mut emu, winapi64::user32::FindWindowA, &[class_addr, 0]);
    assert_eq!(ret, 0, "FindWindowA must return NULL (window not found)");
}

// ── user32: GetKeyState / GetAsyncKeyState ──

#[test]
fn test_get_key_state_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::user32::GetKeyState,
        &[0x41], // VK_A
    );
    assert_eq!(ret, 0, "GetKeyState must return 0 (not pressed)");

    let ret = helpers::call_winapi64(&mut emu, winapi64::user32::GetAsyncKeyState, &[0x41]);
    assert_eq!(ret, 0, "GetAsyncKeyState must return 0");
}

// ── user32: SetWindowsHookExA ──

#[test]
fn test_set_windows_hook_ex_a_64() {
    helpers::setup();
    let mut emu = emu64();

    // WH_KEYBOARD_LL = 13
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::user32::SetWindowsHookExA,
        &[13, 0x401000, 0, 0],
    );
    assert_ne!(ret, 0, "SetWindowsHookExA must return a hook handle");
}

// ── ole32: CoInitializeEx + CoTaskMemAlloc + CoTaskMemFree + CoUninitialize ──

#[test]
fn test_com_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoInitializeEx,
        &[0, 0x2], // COINIT_APARTMENTTHREADED
    );
    assert_eq!(ret, 0, "CoInitializeEx must return S_OK");

    let ptr = helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemAlloc, &[0x200]);
    assert_ne!(ptr, 0, "CoTaskMemAlloc returned NULL");
    assert!(emu.maps.is_mapped(ptr), "CoTaskMemAlloc pointer not mapped");

    emu.maps.write_dword(ptr, 0xBAADF00D);
    assert_eq!(emu.maps.read_dword(ptr).unwrap(), 0xBAADF00D);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoTaskMemFree, &[ptr]);

    helpers::call_winapi64(&mut emu, winapi64::ole32::CoUninitialize, &[]);
}

// ── ole32: CoCreateInstance returns REGDB_E_CLASSNOTREG ──

#[test]
fn test_co_create_instance_64() {
    helpers::setup();
    let mut emu = emu64();

    let clsid_addr = 0x100000u64;
    let iid_addr = 0x100020u64;
    let ppv_addr = 0x100040u64;
    emu.maps
        .create_map("com_io", 0x100000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(ppv_addr, 0xDEAD);

    // CoCreateInstance(rclsid, pUnkOuter, dwClsContext, riid, ppv)
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ole32::CoCreateInstance,
        &[clsid_addr, 0, 1, iid_addr, ppv_addr],
    );
    assert_eq!(
        ret, 0x80040154,
        "CoCreateInstance must return REGDB_E_CLASSNOTREG"
    );
    let ppv = emu.maps.read_qword(ppv_addr).unwrap();
    assert_eq!(ppv, 0, "ppv must be set to NULL");
}

// ── bcrypt: BCryptOpenAlgorithmProvider + BCryptGenRandom + BCryptCloseAlgorithmProvider ──

#[test]
fn test_bcrypt_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let handle_addr = 0x100000u64;
    let buf_addr = 0x100100u64;
    let algo_addr = 0x200000u64;
    emu.maps
        .create_map("bcrypt_io", 0x100000, 0x1000, Permission::READ_WRITE);
    emu.maps
        .create_map("bcrypt_str", 0x200000, 0x1000, Permission::READ_WRITE);

    // Write L"AES" (wide)
    let aes: [u8; 8] = [b'A', 0, b'E', 0, b'S', 0, 0, 0];
    for (i, &b) in aes.iter().enumerate() {
        emu.maps.write_byte(algo_addr + i as u64, b);
    }

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::bcrypt::BCryptOpenAlgorithmProvider,
        &[handle_addr, algo_addr, 0, 0],
    );
    assert_eq!(
        ret, 0,
        "BCryptOpenAlgorithmProvider must return STATUS_SUCCESS"
    );
    let h_algo = emu.maps.read_qword(handle_addr).unwrap();
    assert_ne!(h_algo, 0, "algorithm handle must be non-zero");

    // BCryptGenRandom: fill 32 bytes
    for i in 0..32u64 {
        emu.maps.write_byte(buf_addr + i, 0);
    }
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::bcrypt::BCryptGenRandom,
        &[h_algo, buf_addr, 32, 0],
    );
    assert_eq!(ret, 0, "BCryptGenRandom must return STATUS_SUCCESS");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::bcrypt::BCryptCloseAlgorithmProvider,
        &[h_algo, 0],
    );
    assert_eq!(
        ret, 0,
        "BCryptCloseAlgorithmProvider must return STATUS_SUCCESS"
    );
}

// ── winhttp: full HTTP flow ──

#[test]
fn test_winhttp_full_flow_64() {
    helpers::setup();
    let mut emu = emu64();

    let agent_addr = 0x100000u64;
    let server_addr = 0x100100u64;
    let verb_addr = 0x100200u64;
    let path_addr = 0x100300u64;
    let bytes_read_addr = 0x100400u64;
    let buffer_addr = 0x100500u64;
    emu.maps
        .create_map("http_io", 0x100000, 0x1000, Permission::READ_WRITE);

    // Write L"TestAgent"
    let agent: [u8; 20] = [
        b'T', 0, b'e', 0, b's', 0, b't', 0, b'A', 0, b'g', 0, b'e', 0, b'n', 0, b't', 0, 0, 0,
    ];
    for (i, &b) in agent.iter().enumerate() {
        emu.maps.write_byte(agent_addr + i as u64, b);
    }
    // Write L"example.com"
    let server: [u8; 24] = [
        b'e', 0, b'x', 0, b'a', 0, b'm', 0, b'p', 0, b'l', 0, b'e', 0, b'.', 0, b'c', 0, b'o', 0,
        b'm', 0, 0, 0,
    ];
    for (i, &b) in server.iter().enumerate() {
        emu.maps.write_byte(server_addr + i as u64, b);
    }
    // Write L"GET"
    let get: [u8; 8] = [b'G', 0, b'E', 0, b'T', 0, 0, 0];
    for (i, &b) in get.iter().enumerate() {
        emu.maps.write_byte(verb_addr + i as u64, b);
    }
    // Write L"/api"
    let path: [u8; 10] = [b'/', 0, b'a', 0, b'p', 0, b'i', 0, 0, 0];
    for (i, &b) in path.iter().enumerate() {
        emu.maps.write_byte(path_addr + i as u64, b);
    }

    // WinHttpOpen(agent, accessType, proxy, bypass, flags)
    let h_session = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpOpen,
        &[agent_addr, 0, 0, 0, 0],
    );
    assert_ne!(h_session, 0, "WinHttpOpen must return a session handle");

    // WinHttpConnect(session, server, port, reserved)
    let h_connect = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpConnect,
        &[h_session, server_addr, 443, 0],
    );
    assert_ne!(
        h_connect, 0,
        "WinHttpConnect must return a connection handle"
    );

    // WinHttpOpenRequest(connect, verb, objectName, version, referrer, acceptTypes, flags)
    let h_request = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpOpenRequest,
        &[h_connect, verb_addr, path_addr, 0, 0, 0, 0x00800000],
    );
    assert_ne!(
        h_request, 0,
        "WinHttpOpenRequest must return a request handle"
    );

    // WinHttpSendRequest
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpSendRequest,
        &[h_request, 0, 0, 0, 0, 0, 0],
    );
    assert_eq!(ret, 1, "WinHttpSendRequest must return TRUE");

    // WinHttpReceiveResponse
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpReceiveResponse,
        &[h_request, 0],
    );
    assert_eq!(ret, 1, "WinHttpReceiveResponse must return TRUE");

    // WinHttpReadData
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpReadData,
        &[h_request, buffer_addr, 0x100, bytes_read_addr],
    );
    assert_eq!(ret, 1, "WinHttpReadData must return TRUE");
    let bytes = emu.maps.read_dword(bytes_read_addr).unwrap();
    assert_eq!(bytes, 0, "bytes read must be 0 (stub)");

    // WinHttpCloseHandle — close all in reverse order
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpCloseHandle,
        &[h_request],
    );
    assert_eq!(ret, 1, "WinHttpCloseHandle(request) must return TRUE");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpCloseHandle,
        &[h_connect],
    );
    assert_eq!(ret, 1, "WinHttpCloseHandle(connect) must return TRUE");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::winhttp::WinHttpCloseHandle,
        &[h_session],
    );
    assert_eq!(ret, 1, "WinHttpCloseHandle(session) must return TRUE");
}
