use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::{helper, winapi64};
use crate::windows::constants::errors::*;
use crate::windows::constants::process::*;
use crate::windows::constants::system::*;

use crate::{emu32, emu64};
use itertools::iproduct;

const PROCESS_BASIC_INFORMATION_SIZE: u64 = 48;
const SYSTEM_BASIC_INFORMATION_SIZE: u64 = 64;

#[test]
fn close_handle_32() {
    helpers::setup();
    let mut emu = emu32();

    let handle = helper::handler_create("dummy_file");

    let ret = helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[handle]);
    assert_eq!(ret, 1, "CloseHandle should return TRUE");
}

#[test]
fn close_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    let handle = helper::handler_create("dummy_file");

    let ret = helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[handle]);

    assert_eq!(ret, 1);
}

#[test]
fn is_wow64_process_64() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("wow64", out_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(out_addr, 0xDEAD);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::IsWow64Process,
        &[NT_CURRENT_PROCESS, out_addr],
    );
    assert_eq!(ret, 1, "IsWow64Process must return TRUE");
    let wow64 = emu.maps.read_qword(out_addr).unwrap();
    assert_eq!(wow64, 0, "64-bit process must not be WoW64");
}

#[test]
fn duplicate_handle_64() {
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
            NT_CURRENT_PROCESS,
            src_handle, // hSourceHandle
            NT_CURRENT_PROCESS,
            out_addr, // lpTargetHandle
            0,        // dwDesiredAccess
            0,        // bInheritHandle
            0,        // dwOptions
        ],
    );
    assert_eq!(ret, 1, "DuplicateHandle must return TRUE");
    let dup = emu.maps.read_qword(out_addr).unwrap();
    assert_eq!(dup, src_handle, "duplicated handle must equal source");
}

#[test]
fn set_environment_variable_a_64() {
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

#[test]
fn nt_query_information_process_64() {
    helpers::setup();
    let mut emu = emu64();

    let info_addr = 0x100000u64;
    let ret_len_addr = 0x100100u64;
    emu.maps
        .create_map("nqip_io", 0x100000, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(info_addr, 0xDEAD);

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtQueryInformationProcess,
        &[
            NT_CURRENT_PROCESS,
            PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION,
            info_addr,
            PROCESS_BASIC_INFORMATION_SIZE,
            ret_len_addr,
        ],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtQueryInformationProcess must return STATUS_SUCCESS"
    );
}

#[test]
fn adjust_token_privileges_64() {
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

#[test]
fn is_wow64_process() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("wow64out", out_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::IsWow64Process,
        &[NT_CURRENT_PROCESS, out_addr],
    );
    assert_eq!(ret, 1, "IsWow64Process should return TRUE");
    assert_eq!(
        emu.maps.read_qword(out_addr).unwrap(),
        0,
        "64-bit process should report WoW64 = FALSE"
    );
}

#[test]
fn duplicate_handle() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("dupout", out_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::DuplicateHandle,
        &[NT_CURRENT_PROCESS, 0x42, NT_CURRENT_PROCESS, out_addr],
    );
    assert_eq!(ret, 1, "DuplicateHandle should return TRUE");
    assert_eq!(
        emu.maps.read_qword(out_addr).unwrap(),
        0x42,
        "duplicated handle should match source"
    );
}

#[test]
fn set_environment_variable_a() {
    helpers::setup();
    let mut emu = emu64();

    let name_addr = 0x100000u64;
    let value_addr = 0x100100u64;
    emu.maps
        .create_map("envvar", name_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_addr, "MY_VAR");
    emu.maps.write_string(value_addr, "MY_VALUE");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::SetEnvironmentVariableA,
        &[name_addr, value_addr],
    );
    assert_eq!(ret, 1, "SetEnvironmentVariableA should return TRUE");
}

#[test]
fn output_debug_string_a() {
    helpers::setup();
    let mut emu = emu64();

    let msg_addr = 0x100000u64;
    emu.maps
        .create_map("dbgmsg", msg_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(msg_addr, "Debug message");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::OutputDebugStringA,
        &[msg_addr],
    );
    assert_eq!(ret, 0);
}

#[test]
fn nt_query_information_process() {
    helpers::setup();
    let mut emu = emu64();

    let buf_addr = 0x100000u64;
    let ret_len_addr = 0x100100u64;
    emu.maps
        .create_map("qip", buf_addr, 0x1000, Permission::READ_WRITE);

    // NtQueryInformationProcess(handle, class, buffer, length, return_length_ptr)
    // return_length_ptr is the 5th arg (at rsp+0x20)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtQueryInformationProcess,
        &[
            NT_CURRENT_PROCESS,
            PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION,
            buf_addr,
            PROCESS_BASIC_INFORMATION_SIZE,
            ret_len_addr,
        ],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtQueryInformationProcess should return STATUS_SUCCESS"
    );

    let ret_len = emu.maps.read_dword(ret_len_addr).unwrap();
    assert_eq!(
        ret_len as u64, PROCESS_BASIC_INFORMATION_SIZE,
        "returned length should be 48 for ProcessBasicInformation"
    );
}

#[test]
fn nt_query_system_information() {
    helpers::setup();
    let mut emu = emu64();

    let buf_addr = 0x100000u64;
    let ret_len_addr = 0x100100u64;
    emu.maps
        .create_map("qsi", buf_addr, 0x1000, Permission::READ_WRITE);

    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtQuerySystemInformation,
        &[
            SYSTEM_BASIC_INFORMATION,
            buf_addr,
            SYSTEM_BASIC_INFORMATION_SIZE,
            ret_len_addr,
        ],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtQuerySystemInformation should return STATUS_SUCCESS"
    );

    let ret_len = emu.maps.read_dword(ret_len_addr).unwrap();
    assert_eq!(ret_len as u64, SYSTEM_BASIC_INFORMATION_SIZE);
}

#[test]
fn adjust_token_privileges() {
    helpers::setup();
    let mut emu = emu64();

    let data_addr = 0x100000u64;
    emu.maps
        .create_map("tokpriv", data_addr, 0x1000, Permission::READ_WRITE);

    emu.maps.write_qword(emu.regs().rsp + 0x20, 0);
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::advapi32::AdjustTokenPrivileges,
        &[0x1234, 0, data_addr, 0],
    );
    assert_eq!(ret, 1, "AdjustTokenPrivileges should return TRUE");
}

#[test]
fn current_handles_have_guest_pointer_width_and_preserve_last_error() {
    const LAST_ERROR_MARKER: u64 = 1234;
    for (is64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        for (api, handle) in [
            ("GetCurrentProcess", NT_CURRENT_PROCESS),
            ("GetCurrentThread", NT_CURRENT_THREAD),
        ] {
            emu.set_last_error(LAST_ERROR_MARKER);
            let expected = if is64 { handle } else { handle as u32 as u64 };
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, api, &[]),
                expected
            );
            assert_eq!(emu.get_last_error(), LAST_ERROR_MARKER);
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, "CloseHandle", &[expected]),
                1
            );
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, api, &[]),
                expected
            );
        }
    }
}

#[test]
fn toolhelp_next_reports_exhaustion_without_changing_the_output() {
    const TH32CS_SNAPPROCESS: u64 = 0x0000_0002;
    const TH32CS_SNAPMODULE: u64 = 0x0000_0008;
    const OUTPUT_BYTES: usize = 1080;
    for is64 in [false, true] {
        let (mut emu, entry) = helpers::prepare_winapi(is64);
        for (api, flags) in [
            ("Module32Next", TH32CS_SNAPMODULE),
            ("Module32NextW", TH32CS_SNAPMODULE),
            ("Process32Next", TH32CS_SNAPPROCESS),
            ("Process32NextW", TH32CS_SNAPPROCESS),
        ] {
            let snapshot = helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "CreateToolhelp32Snapshot",
                &[flags, 0],
            );
            let expected = vec![0xcc; OUTPUT_BYTES];
            assert!(emu.maps.write_bytes(entry, &expected));
            emu.set_last_error(ERROR_SUCCESS);
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernel32.dll", api, &[snapshot, entry]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_NO_MORE_FILES, "{api}");
            assert_eq!(
                emu.maps.read_bytes(entry, expected.len()).as_ref(),
                expected
            );
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernel32.dll", api, &[0, entry]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE, "{api}");
            assert_eq!(
                emu.maps.read_bytes(entry, expected.len()).as_ref(),
                expected
            );
            helper::handler_close(snapshot);
        }
    }
}
