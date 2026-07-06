use crate::tests::helpers;
use crate::winapi::winapi32;
use crate::windows::constants;
use crate::*;

#[test]
// test 32bits allocators
pub fn allocator32_test() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    emu.maps.clear();
    emu.init_win32(false, false);

    assert_eq!(emu.maps.exists_mapname("shell32.rsrc"), true);
    assert_eq!(emu.maps.get_map_by_name("shell32.rsrc").is_some(), true);
    assert_eq!(emu.maps.exists_mapname("notexist"), false);
    assert_eq!(emu.maps.get_map_by_name("notexist").is_some(), false);

    for _ in 0..700 {
        assert_eq!(emu.maps.alloc(1024).is_some(), true);
        assert_eq!(emu.maps.lib32_alloc(1024).is_some(), true);
    }

    assert_eq!(emu.maps.mem_test(), true);

    // VirtualAlloc(addr, sz, flAllocationType, flProtect)
    let p = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAlloc,
        &[0, 1024, constants::MEM_RESERVE, 0x40],
    );
    assert_eq!(emu.maps.is_allocated(p as u64), true);

    helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAlloc,
        &[
            0x30000000,
            1024,
            constants::MEM_RESERVE | constants::MEM_COMMIT,
            0x40,
        ],
    );

    let committed = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAlloc,
        &[0x30000000, 1024, constants::MEM_COMMIT, 0x40],
    );
    assert_eq!(committed, 0x30000000);

    assert!(emu.maps.is_allocated(0x30000000));
    assert!(emu.maps.mem_test());
}

#[test]
fn virtual_alloc_zero_size_32() {
    helpers::setup();
    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    emu.maps.clear();
    emu.init_win32(false, false);

    let r = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAlloc,
        &[0, 0, constants::MEM_COMMIT, 0x40],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi32(&mut emu, winapi32::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER as u32
    );

    let r = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAllocEx,
        &[0xffff_ffff, 0, 0, constants::MEM_COMMIT, 0x40],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi32(&mut emu, winapi32::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER as u32
    );

    let r = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAllocExNuma,
        &[0xffff_ffff, 0, 0, constants::MEM_COMMIT, 0x40, 0],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi32(&mut emu, winapi32::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER as u32
    );
}

#[test]
fn virtual_alloc_commit_unmapped_32() {
    helpers::setup();
    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    emu.maps.clear();
    emu.init_win32(false, false);

    // MEM_COMMIT at a fresh, never-mapped fixed address must fail.
    let r = helpers::call_winapi32(
        &mut emu,
        winapi32::kernel32::VirtualAlloc,
        &[0x40000000, 0x1000, constants::MEM_COMMIT, 0x40],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi32(&mut emu, winapi32::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER as u32
    );
}
