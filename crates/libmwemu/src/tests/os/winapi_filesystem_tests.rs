use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::process::*;
use crate::windows::constants::security::*;
use itertools::iproduct;

use crate::maps::mem64::Permission;
use crate::tests::helpers::file_system_root;
use crate::tests::helpers::{self, call_winapi_export as call, prepare_winapi as prepare};
use crate::winapi::winapi64;

use crate::{emu32, emu64};
use std::fs;

const MARKER: u32 = 0xcccc_cccc;

#[test]
fn file_apis_share_handles_data_positions_and_dword_output_counts() {
    let host_dir = file_system_root().join("c/api-contracts");
    fs::create_dir_all(&host_dir).unwrap();
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let filename = format!("file-{is64}-{library}-{wide}.bin");
        let host = host_dir.join(&filename);
        fs::write(&host, b"abc").unwrap();
        let guest = format!("C:\\api-contracts\\{filename}");
        let create = if wide {
            emu.maps.write_wide_string(data, &guest);
            "CreateFileW"
        } else {
            emu.maps.write_string(data, &guest);
            "CreateFileA"
        };
        let handle = call(
            &mut emu,
            library,
            create,
            &[
                data,
                (GENERIC_READ | GENERIC_WRITE) as u64,
                (FILE_SHARE_READ | FILE_SHARE_WRITE) as u64,
                0,
                OPEN_EXISTING as u64,
                0,
                0,
            ],
        );
        assert_ne!(handle as u32, u32::MAX);
        let output = data + 0x200;
        let buffer = data + 0x300;
        assert_eq!(call(&mut emu, library, "GetFileSize", &[handle, output]), 3);
        assert_eq!(emu.maps.read_dword(output), Some(0));
        assert_eq!(
            call(&mut emu, library, "GetFileSizeEx", &[handle, output]),
            1
        );
        assert_eq!(emu.maps.read_qword(output), Some(3));
        emu.maps.memset(buffer, 0xcc, 8);
        emu.maps.write_dword(output + 4, MARKER);
        assert_eq!(
            call(
                &mut emu,
                library,
                "ReadFile",
                &[handle, buffer, 8, output, 0]
            ),
            1
        );
        assert_eq!(
            emu.maps.read_bytes(buffer, 8).as_ref(),
            b"abc\xcc\xcc\xcc\xcc\xcc"
        );
        assert_eq!(emu.maps.read_dword(output), Some(3));
        assert_eq!(emu.maps.read_dword(output + 4), Some(MARKER));
        assert_eq!(
            call(
                &mut emu,
                library,
                "ReadFile",
                &[handle, buffer, 8, output, 0]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(output), Some(0));
        assert_eq!(
            call(
                &mut emu,
                library,
                "SetFilePointer",
                &[handle, -1i64 as u64, 0, FILE_END as u64]
            ),
            2
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "WriteFile",
                &[handle, buffer, 3, output, 0]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(output), Some(3));
        assert_eq!(emu.maps.read_dword(output + 4), Some(MARKER));
        assert_eq!(call(&mut emu, library, "GetFileSize", &[handle, 0]), 5);
        assert_eq!(fs::read(&host).unwrap(), b"ababc");
        let seek_args = if is64 {
            vec![handle, -2i64 as u64, output, FILE_CURRENT as u64]
        } else {
            vec![
                handle,
                -2i32 as u32 as u64,
                u32::MAX as u64,
                output,
                FILE_CURRENT as u64,
            ]
        };
        assert_eq!(call(&mut emu, library, "SetFilePointerEx", &seek_args), 1);
        assert_eq!(emu.maps.read_qword(output), Some(3));
        assert_eq!(call(&mut emu, library, "SetEndOfFile", &[handle]), 1);
        assert_eq!(call(&mut emu, library, "FlushFileBuffers", &[handle]), 1);
        assert_eq!(fs::read(&host).unwrap(), b"aba");
        assert_eq!(call(&mut emu, library, "GetFileSize", &[handle, 0]), 3);
        // Failure must not move the file pointer or modify output memory.
        emu.maps.write_dword(output, MARKER);
        assert_eq!(
            call(
                &mut emu,
                library,
                "SetFilePointer",
                &[handle, -4i64 as u64, 0, FILE_CURRENT as u64]
            ),
            u32::MAX as u64
        );
        assert_eq!(emu.get_last_error(), ERROR_NEGATIVE_SEEK);
        assert_eq!(
            call(
                &mut emu,
                library,
                "ReadFile",
                &[handle, buffer, 1, output, data]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NOT_SUPPORTED);
        assert_eq!(call(&mut emu, library, "GetFileSizeEx", &[handle, 0]), 0);
        // A valid low DWORD equal to the error sentinel must clear LastError.
        emu.maps.write_dword(output, 0);
        emu.set_last_error(MARKER as u64);
        assert_eq!(
            call(
                &mut emu,
                library,
                "SetFilePointer",
                &[handle, u32::MAX as u64, output, FILE_BEGIN as u64]
            ),
            u32::MAX as u64
        );
        assert_eq!(emu.get_last_error(), ERROR_SUCCESS);
        assert_eq!(call(&mut emu, library, "CloseHandle", &[handle]), 1);
        emu.maps.write_dword(output, MARKER);
        assert_eq!(
            call(&mut emu, library, "GetFileSize", &[handle, output]),
            u32::MAX as u64
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        assert_eq!(emu.maps.read_dword(output), Some(MARKER));
        assert_eq!(
            call(&mut emu, library, "GetFileSizeEx", &[handle, output]),
            0
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "ReadFile",
                &[handle, buffer, 1, output, 0]
            ),
            0
        );
        assert_eq!(emu.maps.read_dword(output), Some(0));
        assert_eq!(call(&mut emu, library, "FlushFileBuffers", &[handle]), 0);
        assert_eq!(call(&mut emu, library, "SetEndOfFile", &[handle]), 0);
        let read_only = call(
            &mut emu,
            library,
            create,
            &[
                data,
                GENERIC_READ as u64,
                (FILE_SHARE_READ | FILE_SHARE_WRITE) as u64,
                0,
                OPEN_EXISTING as u64,
                0,
                0,
            ],
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "WriteFile",
                &[read_only, buffer, 1, output, 0]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
        assert_eq!(call(&mut emu, library, "SetEndOfFile", &[read_only]), 0);
        assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
        call(&mut emu, library, "CloseHandle", &[read_only]);
        fs::remove_file(host).unwrap();
        assert_eq!(
            call(
                &mut emu,
                library,
                create,
                &[
                    data,
                    GENERIC_READ as u64,
                    FILE_SHARE_READ as u64,
                    0,
                    OPEN_EXISTING as u64,
                    0,
                    0
                ]
            ) as u32,
            u32::MAX
        );
        assert_eq!(emu.get_last_error(), ERROR_FILE_NOT_FOUND);
    }
}

#[test]
fn write_file_accepts_stdout_handles_and_preserves_the_adjacent_dword() {
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        emu.maps.write_bytes(data, b"ok");
        emu.maps.write_dword(data + 8, MARKER);
        let handle = call(
            &mut emu,
            "kernel32.dll",
            "GetStdHandle",
            &[STD_OUTPUT_HANDLE as u64],
        );
        for library in ["kernel32.dll", "kernelbase.dll"] {
            assert_eq!(
                call(
                    &mut emu,
                    library,
                    "WriteFile",
                    &[handle, data, 2, data + 4, 0]
                ),
                1
            );
            assert_eq!(emu.maps.read_dword(data + 4), Some(2));
            assert_eq!(emu.maps.read_dword(data + 8), Some(MARKER));
        }
    }
}

#[test]
fn write_file_32() {
    helpers::setup();
    let mut emu = emu32();

    // BOOL WriteFile(hFile, lpBuffer, nBytes, lpNumberOfBytesWritten, lpOverlapped)
    let buf_addr = 0x20000u64;
    emu.maps
        .create_map("buffer", buf_addr, 0x2000, Permission::READ_WRITE); // covers written_ptr too
    let written_ptr = 0x21000u64;
    let stdout = call(
        &mut emu,
        "kernel32.dll",
        "GetStdHandle",
        &[STD_OUTPUT_HANDLE as u64],
    );

    let ret = helpers::call_winapi_export(
        &mut emu,
        "kernel32.dll",
        "WriteFile",
        &[stdout, buf_addr, 100, written_ptr, 0],
    );
    assert_eq!(ret, 1, "WriteFile 32-bit failed");

    // Check bytes written
    let bytes = emu
        .maps
        .read_dword(written_ptr)
        .expect("Cannot read bytes written");
    assert_eq!(bytes, 100);
}

#[test]
fn write_file() {
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

    let stdout = call(
        &mut emu,
        "kernel32.dll",
        "GetStdHandle",
        &[STD_OUTPUT_HANDLE as u64],
    );

    // BOOL WriteFile(hFile, lpBuffer, nBytes, lpNumberOfBytesWritten, lpOverlapped)
    // The 5th argument (lpOverlapped = NULL) is passed on the stack at rsp+0x20.
    let ret = helpers::call_winapi_export(
        &mut emu,
        "kernel32.dll",
        "WriteFile",
        &[stdout, buff_addr, 12, written_ptr, 0],
    );

    // RAX should be 1 (TRUE)
    assert_eq!(ret, 1, "WriteFile failed (returned 0)");

    // Read bytes written
    let bytes = emu.maps.read_dword(written_ptr).unwrap();
    assert_eq!(bytes, 12);
}

#[test]
fn create_directory_a_64() {
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

#[test]
fn create_directory_a() {
    helpers::setup();
    let mut emu = emu64();

    let path_addr = 0x100000u64;
    emu.maps
        .create_map("dirpath", path_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(path_addr, "C:\\test_dir");

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::CreateDirectoryA,
        &[path_addr, 0],
    );
    assert_eq!(ret, 1, "CreateDirectoryA stub should return TRUE");
}

#[test]
fn delete_file_w() {
    helpers::setup();
    let mut emu = emu64();

    let path_addr = 0x100000u64;
    emu.maps
        .create_map("filepath", path_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(path_addr, "C:\\test.txt");

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::DeleteFileW, &[path_addr]);
    assert_eq!(ret, 1, "DeleteFileW stub should return TRUE");
}

#[test]
fn disk_geometry_writes_all_four_dword_outputs_including_the_stack_argument() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let api = if wide {
            "GetDiskFreeSpaceW"
        } else {
            "GetDiskFreeSpaceA"
        };
        let outputs = [data, data + 16, data + 32, data + 48];
        for address in outputs {
            emu.maps.write_dword(address, MARKER);
            emu.maps.write_dword(address + 4, MARKER);
        }
        assert_eq!(
            call(
                &mut emu,
                library,
                api,
                &[0, outputs[0], outputs[1], outputs[2], outputs[3]]
            ),
            1
        );
        let values = outputs.map(|address| {
            assert_eq!(emu.maps.read_dword(address + 4), Some(MARKER));
            let value = emu.maps.read_dword(address).unwrap();
            assert_ne!(value, MARKER);
            assert!(value > 0);
            value
        });
        assert!(values[0].is_power_of_two());
        assert!(values[1].is_power_of_two());
        assert!(values[2] <= values[3]);
        for address in outputs {
            emu.maps.write_dword(address, MARKER);
        }
        assert_eq!(
            call(
                &mut emu,
                library,
                api,
                &[0, outputs[0], outputs[1], outputs[2], data + 0x2000]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
        for address in outputs {
            assert_eq!(emu.maps.read_dword(address), Some(MARKER));
        }
    }
}

#[test]
fn current_directory_variants_update_the_same_guest_state() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let path = r"C:\guest-data";
        let api = if wide {
            emu.maps.write_wide_string(data, path);
            "SetCurrentDirectoryW"
        } else {
            emu.maps.write_string(data, path);
            "SetCurrentDirectoryA"
        };
        assert_eq!(call(&mut emu, library, api, &[data]), 1);
        assert_eq!(emu.cfg.cwd_path, path);
        emu.maps.write_word(data, 0);
        assert_eq!(call(&mut emu, library, api, &[data]), 0);
        assert_eq!(emu.cfg.cwd_path, path);
    }
}

#[test]
fn create_file_a_missing_path_returns_invalid_handle() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("file_buf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(buf, "C:\\test.txt");

    let ret = helpers::call_winapi_export(
        &mut emu,
        "kernel32.dll",
        "CreateFileA",
        &[buf, GENERIC_READ as u64, 0, 0, OPEN_EXISTING as u64, 0, 0],
    );
    assert_eq!(
        ret, INVALID_HANDLE_VALUE_64,
        "CreateFileA with a missing path should return INVALID_HANDLE_VALUE"
    );
}

#[test]
fn create_file_w_missing_path_returns_invalid_handle() {
    helpers::setup();
    let mut emu = emu64();

    let buf = 0x30000u64;
    emu.maps
        .create_map("file_buf", buf, 0x1000, Permission::READ_WRITE);
    emu.maps.write_wide_string(buf, "C:\\test.txt");

    let ret = helpers::call_winapi_export(
        &mut emu,
        "kernel32.dll",
        "CreateFileW",
        &[buf, GENERIC_READ as u64, 0, 0, OPEN_EXISTING as u64, 0, 0],
    );
    assert_eq!(
        ret, INVALID_HANDLE_VALUE_64,
        "CreateFileW with a missing path should return INVALID_HANDLE_VALUE"
    );
}

#[test]
fn standard_handles_are_shared_and_usable_by_write_file() {
    for (is64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, data) = prepare(is64);
        for selector in [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
            emu.set_last_error(MARKER as u64);
            let handle = call(&mut emu, library, "GetStdHandle", &[selector as u64]);
            assert_ne!(handle, 0);
            assert_ne!(handle as u32, u32::MAX);
            assert_eq!(emu.get_last_error(), MARKER as u64);
            assert_eq!(
                call(
                    &mut emu,
                    "kernel32.dll",
                    "GetStdHandle",
                    &[selector as i32 as i64 as u64]
                ),
                handle
            );
            if selector != STD_INPUT_HANDLE {
                emu.maps.write_bytes(data, b"abc");
                emu.maps.write_dword(data + 0x104, MARKER);
                assert_eq!(
                    call(
                        &mut emu,
                        library,
                        "WriteFile",
                        &[handle, data, 3, data + 0x100, 0]
                    ),
                    1
                );
                assert_eq!(emu.maps.read_dword(data + 0x100), Some(3));
                assert_eq!(emu.maps.read_dword(data + 0x104), Some(MARKER));
            }
        }
        let invalid = if is64 {
            INVALID_HANDLE_VALUE_64
        } else {
            INVALID_HANDLE_VALUE_32
        };
        assert_eq!(call(&mut emu, library, "GetStdHandle", &[42]), invalid);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
    }
}

#[test]
fn directory_paths_respect_capacity_and_count_encoded_characters() {
    const SENTINEL: u8 = 0xcc;
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let component = if wide { "dir-😀" } else { "dir-ascii" };
        emu.cfg.windows_directory = format!(r"D:\{component}\Windows");
        emu.cfg.system_directory = format!(r"D:\{component}\Windows\System32");
        emu.cfg.temp_path = format!(r"D:\{component}\Temp");
        emu.cfg.cwd_path = format!(r"D:\{component}\Working");
        let cases = [
            (
                "GetWindowsDirectory",
                emu.cfg.windows_directory.clone(),
                false,
            ),
            (
                "GetSystemWindowsDirectory",
                emu.cfg.windows_directory.clone(),
                false,
            ),
            (
                "GetSystemDirectory",
                emu.cfg.system_directory.clone(),
                false,
            ),
            ("GetTempPath", format!("{}\\", emu.cfg.temp_path), true),
            ("GetCurrentDirectory", emu.cfg.cwd_path.clone(), true),
        ];
        for (name, path, size_first) in cases {
            let api = format!("{name}{}", if wide { 'W' } else { 'A' });
            let encoded: Vec<u8> = if wide {
                path.encode_utf16()
                    .chain([0])
                    .flat_map(u16::to_le_bytes)
                    .collect()
            } else {
                path.bytes().chain([0]).collect()
            };
            let length = if wide {
                encoded.len() / size_of::<u16>() - 1
            } else {
                encoded.len() - 1
            };
            let args = |capacity: u64, buffer: u64| {
                if size_first {
                    [capacity, buffer]
                } else {
                    [buffer, capacity]
                }
            };
            for capacity in [0, length, length + 1, length + 8] {
                emu.maps.memset(data, SENTINEL, 256);
                emu.set_last_error(MARKER as u64);
                let written = capacity > length;
                let expected = if written { length } else { length + 1 } as u64;
                assert_eq!(
                    call(&mut emu, library, &api, &args(capacity as u64, data)),
                    expected,
                    "{library}!{api}, capacity={capacity}"
                );
                assert_eq!(emu.get_last_error(), MARKER as u64);
                if written {
                    assert_eq!(emu.maps.read_bytes(data, encoded.len()), encoded);
                }
                let untouched = if written { encoded.len() } else { 0 };
                assert_eq!(
                    emu.maps
                        .read_bytes(data + untouched as u64, 256 - untouched),
                    vec![SENTINEL; 256 - untouched]
                );
            }
            assert_eq!(
                call(&mut emu, library, &api, &args(0, 0)),
                (length + 1) as u64
            );
            if is64 {
                assert_eq!(
                    call(&mut emu, library, &api, &args(1 << u32::BITS | 1, data)),
                    (length + 1) as u64
                );
            }
        }
    }
}

#[test]
fn directory_paths_reject_unwritable_outputs_without_partial_writes() {
    const SENTINEL: u8 = 0xcc;
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let readonly = data + 0x2000;
        emu.maps
            .create_map("readonly_directory", readonly, 0x1000, Permission::READ)
            .unwrap();
        let boundary = data + 0x1000 - 2;
        for (name, size_first) in [
            ("GetWindowsDirectory", false),
            ("GetSystemWindowsDirectory", false),
            ("GetSystemDirectory", false),
            ("GetTempPath", true),
            ("GetCurrentDirectory", true),
        ] {
            let api = format!("{name}{}", if wide { 'W' } else { 'A' });
            for buffer in [0, boundary, readonly] {
                emu.maps.memset(boundary, SENTINEL, 2);
                let args = if size_first {
                    [256, buffer]
                } else {
                    [buffer, 256]
                };
                assert_eq!(call(&mut emu, library, &api, &args), 0, "{library}!{api}");
                assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
                assert_eq!(emu.maps.read_bytes(boundary, 2).as_ref(), &[SENTINEL; 2]);
                assert_eq!(emu.maps.read_byte(readonly), Some(0));
            }
        }
    }
}
