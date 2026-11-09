use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::security::*;
use crate::windows::registry_store::HKEY_LOCAL_MACHINE;

use crate::{emu32, emu64};

#[test]
fn reg_open_query_32() {
    helpers::setup();
    let mut emu = emu32();
    let subkey = 0x40000u64;
    let hkey_out = 0x40100u64;
    let data = 0x40200u64;
    let data_sz = 0x40300u64;
    emu.maps
        .create_map("reg", 0x40000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(subkey, "Software\\Microsoft");
    emu.maps.write_dword(data_sz, 0x40);

    let r = helpers::call_winapi32(
        &mut emu,
        winapi32::advapi32::RegOpenKeyExA,
        &[
            HKEY_LOCAL_MACHINE as u32,
            subkey as u32,
            0,
            KEY_READ,
            hkey_out as u32,
        ],
    );
    assert_eq!(r, 0, "RegOpenKeyExA should return ERROR_SUCCESS");
    assert!(
        emu.maps.read_dword(hkey_out).unwrap() != 0,
        "no key handle written"
    );

    // RegQueryValueExA fills a type and a terminated value.
    let r = helpers::call_winapi32(
        &mut emu,
        winapi32::advapi32::RegQueryValueExA,
        &[0x1234, 0, 0, 0, data as u32, data_sz as u32],
    );
    assert_eq!(r, 0, "RegQueryValueExA should return ERROR_SUCCESS");
}

#[test]
fn reg_open_key_ex_w() {
    helpers::setup();
    let mut emu = emu64();

    let subkey_addr = 0x100000u64;
    let result_addr = 0x100100u64;
    emu.maps
        .create_map("regdata", subkey_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(
        subkey_addr,
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
    );

    let ret = helpers::call_winapi64(
        &mut emu,
        |emu| {
            winapi64::registry::gateway("RegOpenKeyExW", emu);
        },
        &[
            HKEY_LOCAL_MACHINE,
            subkey_addr,
            0,
            KEY_ALL_ACCESS as u64,
            result_addr,
        ],
    );
    assert_eq!(ret, 0, "RegOpenKeyExW should return ERROR_SUCCESS (0)");

    // a key nobody created does not exist
    emu.maps.write_wide_string(subkey_addr, "Software\\Missing");
    let ret = helpers::call_winapi64(
        &mut emu,
        |emu| {
            winapi64::registry::gateway("RegOpenKeyExW", emu);
        },
        &[
            HKEY_LOCAL_MACHINE,
            subkey_addr,
            0,
            KEY_ALL_ACCESS as u64,
            result_addr,
        ],
    );
    assert_eq!(
        ret, 2,
        "RegOpenKeyExW of a missing key is ERROR_FILE_NOT_FOUND"
    );
}
