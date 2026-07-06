use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::windows::constants;
use crate::*;

#[test]
// test 64bits allocators
pub fn allocator64_test() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.init_win32(false, false);

    assert_eq!(emu.maps.exists_mapname("notexist"), false);
    assert_eq!(emu.maps.get_map_by_name("notexist").is_some(), false);

    for _ in 0..700 {
        assert_eq!(emu.maps.alloc(1024).is_some(), true);
        assert_eq!(emu.maps.lib64_alloc(1024).is_some(), true);
    }

    assert_eq!(emu.maps.mem_test(), true);

    emu.maps.clear();

    // VirtualAlloc(addr, sz, flAllocationType, flProtect)
    let p = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[0, 1024, constants::MEM_RESERVE as u64, 0x40],
    );
    assert_eq!(emu.maps.is_allocated(p), true);

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[
            0x30000000,
            1024,
            (constants::MEM_RESERVE | constants::MEM_COMMIT) as u64,
            0x40,
        ],
    );

    let committed = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[0x30000000, 1024, constants::MEM_COMMIT as u64, 0x40],
    );
    assert_eq!(committed, 0x30000000);

    assert_eq!(emu.maps.is_allocated(0x30000000), true);
    assert_eq!(emu.maps.mem_test(), true);
}

#[test]
fn virtual_alloc_zero_size_64() {
    helpers::setup();
    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.init_win32(false, false);

    let r = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[0, 0, constants::MEM_COMMIT as u64, 0x40],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi64(&mut emu, winapi64::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER
    );

    let r = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAllocEx,
        &[
            0xffff_ffff_ffff_ffff,
            0,
            0,
            constants::MEM_COMMIT as u64,
            0x40,
        ],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi64(&mut emu, winapi64::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER
    );

    let r = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAllocExNuma,
        &[
            0xffff_ffff_ffff_ffff,
            0,
            0,
            constants::MEM_COMMIT as u64,
            0x40,
            0,
        ],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi64(&mut emu, winapi64::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER
    );
}

#[test]
fn virtual_alloc_commit_unmapped_64() {
    helpers::setup();
    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.init_win32(false, false);

    // MEM_COMMIT at a fresh, never-mapped fixed address must fail.
    let r = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::VirtualAlloc,
        &[0x40000000, 0x1000, constants::MEM_COMMIT as u64, 0x40],
    );
    assert_eq!(r, 0);
    assert_eq!(
        helpers::call_winapi64(&mut emu, winapi64::kernel32::GetLastError, &[]),
        constants::ERROR_INVALID_PARAMETER
    );
}
