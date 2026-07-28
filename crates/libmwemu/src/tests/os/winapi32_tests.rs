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
    assert_ne!(p1, p2);
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
    assert_ne!(p1, p2);
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
