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

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x100],
    );
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

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x100],
    );
    assert!(p1 != 0);
    emu.maps.write_qword(p1, 0x1122_3344_5566_7788);

    // Grow beyond the small/arena threshold to force a dedicated map.
    let p2 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapReAlloc,
        &[0x1234, 0, p1, 0x20000],
    );
    assert!(p2 != 0);
    assert_ne!(p1, p2);
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

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x100],
    );
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

    let p1 = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::HeapAlloc,
        &[0x1234, 0, 0x100],
    );
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
    assert_ne!(p1, p2);
    assert_eq!(emu.maps.read_qword(p2).unwrap(), 0x9988_7766_5544_3322);

    // Invalid pointer must return 0 and not free a real allocation.
    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::RtlReAllocateHeap,
        &[0x1234, 0, 0xdead, 0x100],
    );
    assert_eq!(ret, 0);
}
