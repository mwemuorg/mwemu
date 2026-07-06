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
