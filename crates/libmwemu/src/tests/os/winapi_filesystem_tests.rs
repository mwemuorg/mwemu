use std::fs;

use itertools::iproduct;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::api::windows::common::guest_files::{self, GuestFileRead};
use crate::api::windows::common::strings;
use crate::emu::Emu;
use crate::emu::object_handle::FileHandle;
use crate::maps::mem64::Permission;
use crate::tests::helpers::{
    self, call_winapi_export as call, file_system_root, prepare_winapi as prepare,
};
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::winapi::policy::ApiMode;
use crate::winapi::{helper, registry_store};
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::memory::{PAGE_READONLY, SEC_COMMIT};
use crate::windows::constants::nls::EN_US_LOCALE;
use crate::windows::constants::process::*;
use crate::windows::constants::security::*;
use crate::windows::constants::shell::*;
use crate::{emu32, emu64};

const MARKER: u32 = 0xcccc_cccc;

const FILE_EXPORT_BASE: u64 = 0x0500_0000;
const FILE_PROVIDER_CODE: u64 = 0x0510_0000;
const FILE_EXPORT_NAMES: [&str; 7] = [
    "NtOpenFile",
    "NtQueryInformationFile",
    "NtReadFile",
    "NtCreateSection",
    "NtMapViewOfSection",
    "NtUnmapViewOfSection",
    "NtClose",
];
const FILE_API_STRIDE: u64 = 32;
const FILE_CONTENT: &[u8] = b"data from the guest file provider";
const FILE_TOKEN: u64 = 0xdead;
const SECTION_TOKEN: u64 = 0xbeef;

#[derive(Clone, Copy, PartialEq)]
enum FileProvider {
    Complete,
    InterruptRead,
    Empty,
    MissingSize,
}

fn file_provider(is64: bool, mode: FileProvider, expected_share: u32) -> (Emu, u64) {
    let (mut emu, data) = prepare(is64);
    for (name, base) in [
        ("ntdll.text", FILE_EXPORT_BASE),
        ("file_provider", FILE_PROVIDER_CODE),
    ] {
        emu.maps
            .create_map(name, base, 4096, Permission::READ_WRITE_EXECUTE)
            .unwrap();
    }
    let mut exports = ExportIndexData {
        export_base: 1,
        number_of_functions: FILE_EXPORT_NAMES.len() as u32,
        ordinal_targets: Vec::new(),
        named_exports: Vec::new(),
    };
    for (i, name) in FILE_EXPORT_NAMES.iter().enumerate() {
        let rva = FILE_API_STRIDE * (i + 1) as u64;
        let export = FILE_EXPORT_BASE + rva;
        let callback = FILE_PROVIDER_CODE + rva;
        // Each export jumps to a guest callback. The test supplies its result before RET.
        emu.maps.write_byte(export, 0xe9);
        emu.maps
            .write_dword(export + 1, (callback - (export + 5)) as u32);
        emu.maps.write_byte(callback, 0xc3);
        exports
            .ordinal_targets
            .push(Some(ExportTarget::Direct { rva: rva as u32 }));
        exports.named_exports.push(NamedExport {
            name: (*name).into(),
            ordinal_index: i as u32,
        });
    }
    helpers::register_export_module(&mut emu, "ntdll.dll", FILE_EXPORT_BASE, &exports);
    let original_pc = FILE_PROVIDER_CODE + 1024;
    emu.regs_mut().rip = original_pc;
    emu.maps.write_byte(original_pc, 0x90);
    emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
        let Some(i) = pc
            .checked_sub(FILE_PROVIDER_CODE)
            .and_then(|rva| rva.checked_div(FILE_API_STRIDE))
            .and_then(|i| i.checked_sub(1))
        else {
            return true;
        };
        let Some(api) = FILE_EXPORT_NAMES.get(i as usize) else {
            return true;
        };
        let arg = |index: usize| {
            if is64 && index < 4 {
                [emu.regs().rcx, emu.regs().rdx, emu.regs().r8, emu.regs().r9][index]
            } else {
                read_pointer(emu, emu.sp() + (index as u64 + 1) * pointer_size(emu)).unwrap()
            }
        };
        match *api {
            "NtOpenFile" => {
                let (file, object, share) = (arg(0), arg(2), arg(4));
                assert_eq!(share, u64::from(expected_share));
                let width = pointer_size(emu);
                let descriptor = read_pointer(emu, object + 2 * width).unwrap();
                let name = read_pointer(emu, descriptor + width).unwrap();
                assert_eq!(emu.maps.read_wide_string(name), r"\??\C:\test.bin");
                assert!(write_pointer(emu, file, FILE_TOKEN));
            }
            "NtQueryInformationFile" => {
                let (file, output, size, class) = (arg(0), arg(2), arg(3), arg(4));
                assert_eq!((file, size, class), (FILE_TOKEN, 24, 5));
                if mode != FileProvider::MissingSize {
                    let size = if mode == FileProvider::Empty {
                        0
                    } else {
                        FILE_CONTENT.len()
                    };
                    assert!(emu.maps.write_qword(output + 8, size as u64));
                }
            }
            "NtReadFile" => {
                let (file, io, buffer, length, offset) = (arg(0), arg(4), arg(5), arg(6), arg(7));
                assert_eq!(file, FILE_TOKEN);
                if mode == FileProvider::InterruptRead {
                    emu.regs_mut().rbx = FILE_TOKEN;
                    emu.stop();
                    return false;
                }
                let offset = emu.maps.read_qword(offset).unwrap() as usize;
                // A successful short read must advance the offset and request the remainder.
                let n = (length as usize).min(7);
                assert!(
                    emu.maps
                        .write_bytes(buffer, &FILE_CONTENT[offset..offset + n])
                );
                assert!(write_pointer(emu, io + pointer_size(emu), n as u64));
                let reads = emu.maps.read_dword(data).unwrap();
                emu.maps.write_dword(data, reads + 1);
            }
            "NtCreateSection" => {
                let (output, kind, file) = (arg(0), arg(5), arg(6));
                assert_eq!((kind, file), (u64::from(SEC_COMMIT), FILE_TOKEN));
                assert!(write_pointer(emu, output, SECTION_TOKEN));
            }
            "NtMapViewOfSection" => {
                let (section, base, size) = (arg(0), arg(2), arg(6));
                assert_eq!(section, SECTION_TOKEN);
                assert!(emu.maps.write_bytes(data + 256, FILE_CONTENT));
                assert!(write_pointer(emu, base, data + 256));
                assert!(write_pointer(emu, size, 512));
            }
            "NtUnmapViewOfSection" => {
                assert_eq!(arg(1), data + 256);
                emu.maps.write_dword(data + 4, 1);
            }
            "NtClose" => {
                let offset = match arg(0) {
                    FILE_TOKEN => 8,
                    SECTION_TOKEN => 12,
                    other => panic!("unexpected handle {other:#x}"),
                };
                let closes = emu.maps.read_dword(data + offset).unwrap();
                emu.maps.write_dword(data + offset, closes + 1);
            }
            _ => unreachable!(),
        }
        emu.regs_mut().rbx = 0x5678;
        emu.regs_mut().rax = STATUS_SUCCESS;
        true
    });
    (emu, data)
}

#[test]
fn redirected_nt_file_reads_preserve_the_caller_and_close_guest_objects() {
    for (is64, use_section) in iproduct!([false, true], [false, true]) {
        let (mut emu, data) = file_provider(is64, FileProvider::Complete, FILE_SHARE_READ);
        let pc = emu.pc();
        let sp = emu.sp();
        emu.regs_mut().rbx = 0x1234;
        let read = if use_section {
            guest_files::read_image_file
        } else {
            guest_files::read_file
        };
        let path = if is64 {
            r"\\.\C:\test.bin"
        } else {
            r"C:\test.bin"
        };
        let result = read(&mut emu, path, FILE_SHARE_READ);
        assert!(matches!(result, GuestFileRead::File(bytes) if bytes == FILE_CONTENT));
        assert_eq!((emu.pc(), emu.sp(), emu.regs().rbx), (pc, sp, 0x1234));
        assert_eq!(
            emu.maps.read_dword(data),
            Some(if use_section {
                0
            } else {
                FILE_CONTENT.len().div_ceil(7) as u32
            })
        );
        assert_eq!(emu.maps.read_dword(data + 4), Some(u32::from(use_section)));
        assert_eq!(emu.maps.read_dword(data + 8), Some(1));
        assert_eq!(emu.maps.read_dword(data + 12), Some(u32::from(use_section)));
    }
}

#[test]
fn object_type_queries_size_and_write_the_guest_string() {
    let dll = "ntdll.dll";
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let file = FileHandle::from_guest_bytes(
            r"C:\type-query".into(),
            b"guest".to_vec(),
            GENERIC_READ,
            0,
            FILE_SHARE_READ,
        );
        let handle = u64::from(emu.handle_management.insert_file_handle(file));
        let width = pointer_size(&emu);
        let header = if is64 { 104 } else { 96 };
        let required = if is64 { 120 } else { 112 };
        let returned = data + 192;
        emu.maps.write_dword(data, MARKER);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "NtQueryObject",
                &[handle, OBJECT_TYPE_INFORMATION as u64, 0, 0, returned]
            ),
            STATUS_INFO_LENGTH_MISMATCH
        );
        assert_eq!(emu.maps.read_dword(returned), Some(required as u32));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "NtQueryObject",
                &[
                    handle,
                    OBJECT_TYPE_INFORMATION as u64,
                    data,
                    required - 1,
                    returned
                ]
            ),
            STATUS_INFO_LENGTH_MISMATCH
        );
        assert_eq!(emu.maps.read_dword(data), Some(MARKER));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "ZwQueryObject",
                &[
                    handle,
                    OBJECT_TYPE_INFORMATION as u64,
                    data,
                    required,
                    returned
                ]
            ),
            STATUS_SUCCESS
        );
        assert_eq!(emu.maps.read_word(data), Some(8));
        assert_eq!(emu.maps.read_word(data + 2), Some(10));
        assert_eq!(read_pointer(&emu, data + width), Some(data + header));
        assert_eq!(emu.maps.read_wide_string(data + header), "File");
        assert_eq!(
            emu.maps.read_dword(returned),
            Some(if is64 { 120 } else { 106 })
        );
        assert_eq!(call(&mut emu, dll, "NtClose", &[handle]), STATUS_SUCCESS);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "NtQueryObject",
                &[
                    handle,
                    OBJECT_TYPE_INFORMATION as u64,
                    data,
                    required,
                    returned
                ]
            ),
            STATUS_INVALID_HANDLE
        );
        let anonymous = if is64 { u64::MAX } else { u64::from(u32::MAX) };
        let section = call(
            &mut emu,
            "kernel32.dll",
            "CreateFileMappingW",
            &[anonymous, 0, 4, 0, 4096, 0],
        );
        assert_ne!(section, 0);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "NtQueryObject",
                &[section, OBJECT_TYPE_INFORMATION as u64, data, required, 0]
            ),
            STATUS_SUCCESS
        );
        assert_eq!(emu.maps.read_wide_string(data + header), "Section");
    }
}

#[test]
fn native_nt_file_queries_and_reads_use_the_open_handle() {
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let file = FileHandle::from_guest_bytes(
            r"C:\sample.bin".into(),
            b"abcde".to_vec(),
            GENERIC_READ,
            0,
            FILE_SHARE_READ,
        );
        let handle = u64::from(emu.handle_management.insert_file_handle(file));
        let (io, output, offset, buffer) = (data, data + 64, data + 128, data + 160);
        let width = pointer_size(&emu);
        let query = |emu: &mut Emu, handle, length| {
            call(
                emu,
                "ntdll.dll",
                "NtQueryInformationFile",
                &[
                    handle,
                    io,
                    output,
                    length,
                    u64::from(FILE_STANDARD_INFORMATION),
                ],
            )
        };
        assert!(write_pointer(&mut emu, io + width, u64::from(MARKER)));
        assert_eq!(query(&mut emu, handle, 23), STATUS_INFO_LENGTH_MISMATCH);
        assert_eq!(query(&mut emu, 0, 24), STATUS_INVALID_HANDLE);
        assert_eq!(read_pointer(&emu, io + width), Some(u64::from(MARKER)));
        assert_eq!(query(&mut emu, handle, 24), STATUS_SUCCESS);
        assert_eq!(read_pointer(&emu, io + width), Some(24));
        assert_eq!(emu.maps.read_qword(output + 8), Some(5));
        assert_eq!(emu.maps.read_dword(output + 16), Some(1));
        let read = |emu: &mut Emu, position| {
            call(
                emu,
                "ntdll.dll",
                "NtReadFile",
                &[handle, 0, 0, 0, io, buffer, 2, position, 0],
            )
        };
        emu.maps.write_qword(offset, 2);
        assert_eq!(read(&mut emu, offset), STATUS_SUCCESS);
        assert_eq!(emu.maps.read_word(buffer), Some(u16::from_le_bytes(*b"cd")));
        assert_eq!(read_pointer(&emu, io + width), Some(2));
        emu.maps.write_qword(offset, 99);
        assert_eq!(read(&mut emu, offset), STATUS_END_OF_FILE);
        assert_eq!(read_pointer(&emu, io + width), Some(0));
        emu.maps.write_qword(offset, (-2i64) as u64);
        assert_eq!(read(&mut emu, offset), STATUS_SUCCESS);
        assert_eq!(emu.maps.read_byte(buffer), Some(b'e'));
        assert_eq!(read_pointer(&emu, io + width), Some(1));
        assert_eq!(read(&mut emu, 0), STATUS_END_OF_FILE);
        emu.maps.write_qword(offset, u64::MAX);
        assert_eq!(read(&mut emu, offset), STATUS_INVALID_PARAMETER);
        assert_eq!(read_pointer(&emu, io), Some(STATUS_END_OF_FILE));
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[handle]),
            STATUS_SUCCESS
        );
        assert_eq!(query(&mut emu, handle, 24), STATUS_INVALID_HANDLE);
    }
}

#[test]
fn nt_file_queries_distinguish_empty_files_from_missing_output() {
    for (is64, mode) in iproduct!(
        [false, true],
        [FileProvider::Empty, FileProvider::MissingSize]
    ) {
        let (mut emu, data) = file_provider(is64, mode, FILE_SHARE_READ);
        let result = guest_files::read_file(&mut emu, r"C:\test.bin", FILE_SHARE_READ);
        if mode == FileProvider::Empty {
            assert!(matches!(result, GuestFileRead::File(bytes) if bytes.is_empty()));
        } else {
            assert!(matches!(result, GuestFileRead::Failed(STATUS_FILE_INVALID)));
        }
        assert_eq!(emu.maps.read_dword(data), Some(0));
        assert_eq!(emu.maps.read_dword(data + 8), Some(1));
    }
}

#[test]
fn stopped_nt_file_callback_preserves_the_guest_frame() {
    for is64 in [false, true] {
        let (mut emu, data) = file_provider(is64, FileProvider::InterruptRead, FILE_SHARE_READ);
        let original_pc = emu.pc();
        let original_sp = emu.sp();
        assert!(matches!(
            guest_files::read_file(&mut emu, r"C:\test.bin", FILE_SHARE_READ),
            GuestFileRead::Stopped
        ));
        assert_ne!(emu.pc(), original_pc);
        assert_ne!(emu.sp(), original_sp);
        assert_eq!(emu.regs().rbx, FILE_TOKEN);
        assert!(emu.process_terminated);
        assert_eq!(emu.maps.read_dword(data + 8), Some(0));
    }
}

#[test]
fn win32_file_and_loader_ask_the_guest_provider_for_the_same_path() {
    for is64 in [false, true] {
        let (mut emu, data) = file_provider(is64, FileProvider::Complete, FILE_SHARE_READ);
        let path = data + 1024;
        let buffer = data + 512;
        let count = data + 128;
        emu.maps.write_wide_string(path, r"C:\test.bin");
        let file = call(
            &mut emu,
            "kernel32.dll",
            "CreateFileW",
            &[
                path,
                u64::from(GENERIC_READ),
                u64::from(FILE_SHARE_READ),
                0,
                OPEN_EXISTING as u64,
                FILE_ATTRIBUTE_NORMAL as u64,
                0,
            ],
        );
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "ReadFile",
                &[file, buffer, FILE_CONTENT.len() as u64, count, 0]
            ),
            1
        );
        assert_eq!(
            emu.maps
                .try_read_bytes(buffer, FILE_CONTENT.len())
                .unwrap()
                .as_ref(),
            FILE_CONTENT
        );
        assert_eq!(call(&mut emu, "kernel32.dll", "CloseHandle", &[file]), 1);

        let (mut emu, data) = file_provider(
            is64,
            FileProvider::Complete,
            FILE_SHARE_READ | FILE_SHARE_DELETE,
        );
        let path = data + 1024;
        emu.maps.write_wide_string(path, r"C:\test.bin");
        assert_eq!(call(&mut emu, "kernel32.dll", "LoadLibraryW", &[path]), 0);
        // The guest file exists, but its contents are not a PE image.
        assert_eq!(emu.get_last_error(), ERROR_BAD_EXE_FORMAT);
        assert_eq!(emu.maps.read_dword(data + 4), Some(1));
        assert_eq!(emu.maps.read_dword(data + 12), Some(1));
    }
}

#[test]
fn read_only_guest_file_shares_bytes_with_section_without_moving_the_cursor() {
    let library = "kernel32.dll";
    let contents = b"mapped file data";
    for is64 in [false, true] {
        let (mut emu, buffer) = prepare(is64);
        let count = buffer + 64;
        let file = FileHandle::from_guest_bytes(
            r"C:\private\data.bin".into(),
            contents.to_vec(),
            GENERIC_READ,
            FILE_ATTRIBUTE_NORMAL as u32,
            FILE_SHARE_READ,
        );
        let handle = u64::from(emu.handle_management.insert_file_handle(file));
        assert_eq!(
            call(
                &mut emu,
                library,
                "ReadFile",
                &[handle, buffer, 3, count, 0]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(count), Some(3));
        assert_eq!(
            emu.maps.try_read_bytes(buffer, 3).unwrap().as_ref(),
            &contents[..3]
        );

        let section = call(
            &mut emu,
            library,
            "CreateFileMappingW",
            &[handle, 0, PAGE_READONLY as u64, 0, 0, 0],
        );
        assert_ne!(section, 0);
        let view = call(
            &mut emu,
            library,
            "MapViewOfFile",
            &[section, u64::from(FILE_MAP_READ), 0, 0, 0],
        );
        assert_ne!(view, 0);
        assert_eq!(
            emu.maps
                .try_read_bytes(view, contents.len())
                .unwrap()
                .as_ref(),
            contents
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "ReadFile",
                &[handle, buffer, 32, count, 0]
            ),
            1
        );
        assert_eq!(
            emu.maps.read_dword(count),
            Some((contents.len() - 3) as u32)
        );
        assert_eq!(
            emu.maps
                .try_read_bytes(buffer, contents.len() - 3)
                .unwrap()
                .as_ref(),
            &contents[3..]
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "ReadFile",
                &[handle, buffer, 32, count, 0]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(count), Some(0));
        assert_eq!(call(&mut emu, library, "CloseHandle", &[handle]), 1);
        assert_eq!(call(&mut emu, library, "CloseHandle", &[section]), 1);
        assert_eq!(emu.maps.read_byte(view), Some(contents[0]));
        assert_eq!(call(&mut emu, library, "UnmapViewOfFile", &[view]), 1);
    }
}

#[test]
fn file_queries_agree_on_existing_and_missing_guest_paths() {
    let directory = file_system_root().join("c/attribute-queries");
    fs::create_dir_all(&directory).unwrap();
    fs::write(directory.join("data.bin"), b"1234567").unwrap();
    for (is64, wide) in iproduct!([false, true], [false, true]) {
        let (mut emu, data) = prepare(is64);
        let out = data + 256;
        let names = if wide {
            [
                "GetFileAttributesW",
                "GetFileAttributesExW",
                "PathFileExistsW",
            ]
        } else {
            [
                "GetFileAttributesA",
                "GetFileAttributesExA",
                "PathFileExistsA",
            ]
        };
        for (path, found, directory, error) in [
            (r"C:\attribute-queries\data.bin", true, false, 0),
            (r"C:\attribute-queries", true, true, 0),
            (
                r"C:\attribute-queries\absent",
                false,
                false,
                ERROR_FILE_NOT_FOUND,
            ),
            (
                r"C:\attribute-queries\absent\child",
                false,
                false,
                ERROR_PATH_NOT_FOUND,
            ),
            ("", false, false, ERROR_PATH_NOT_FOUND),
        ] {
            if wide {
                emu.maps.write_wide_string(data, path);
            } else {
                emu.maps.write_string(data, path);
            }
            let expected_error = if found { u64::from(MARKER) } else { error };
            if wide {
                emu.maps.write_bytes(out, &[0xa5; 40]);
                emu.set_last_error(u64::from(MARKER));
                let ok = call(
                    &mut emu,
                    "kernelbase.dll",
                    "GetFileAttributesExFromAppW",
                    &[data, u64::from(GET_FILEEX_INFO_STANDARD), out],
                );
                assert_eq!(ok, u64::from(found));
                assert_eq!(
                    emu.get_last_error(),
                    if found { ERROR_SUCCESS } else { error }
                );
                if found && !directory {
                    assert_eq!(emu.maps.read_dword(out + 32), Some(7));
                } else if !found {
                    assert_eq!(emu.maps.read_dword(out), Some(0xa5a5_a5a5));
                }
                assert_eq!(emu.maps.read_dword(out + 36), Some(0xa5a5_a5a5));
            }
            for library in ["kernel32.dll", "kernelbase.dll"] {
                emu.set_last_error(u64::from(MARKER));
                let attributes = call(&mut emu, library, names[0], &[data]);
                assert_eq!(emu.get_last_error(), expected_error);
                assert_eq!(attributes != u64::from(INVALID_FILE_ATTRIBUTES), found);
                emu.maps.write_bytes(out, &[0xa5; 40]);
                let ok = call(
                    &mut emu,
                    library,
                    names[1],
                    &[data, u64::from(GET_FILEEX_INFO_STANDARD), out],
                );
                assert_eq!(ok, u64::from(found));
                assert_eq!(emu.get_last_error(), expected_error);
                assert_eq!(emu.maps.read_dword(out + 36), Some(0xa5a5_a5a5));
                if found {
                    assert_eq!(emu.maps.read_dword(out), Some(attributes as u32));
                    assert_eq!(attributes & FILE_ATTRIBUTE_DIRECTORY as u64 != 0, directory);
                    if !directory {
                        assert_eq!(emu.maps.read_dword(out + 28), Some(0));
                        assert_eq!(emu.maps.read_dword(out + 32), Some(7));
                    }
                } else {
                    assert_eq!(emu.maps.read_dword(out), Some(0xa5a5_a5a5));
                }
            }
            emu.set_last_error(u64::from(MARKER));
            assert_eq!(
                call(&mut emu, "shlwapi.dll", names[2], &[data]),
                u64::from(found)
            );
            assert_eq!(emu.get_last_error(), expected_error);
        }
        emu.maps.write_bytes(out, &[0xa5; 40]);
        assert_eq!(call(&mut emu, "kernel32.dll", names[1], &[0, 1, out]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
        assert_eq!(emu.maps.read_dword(out), Some(0xa5a5_a5a5));
        emu.set_last_error(u64::from(MARKER));
        assert_eq!(call(&mut emu, "shlwapi.dll", names[2], &[0]), 0);
        assert_eq!(emu.get_last_error(), u64::from(MARKER));
    }
}

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
        let legacy = "GetFileInformationByHandle";
        let extended = "GetFileInformationByHandleEx";
        emu.maps.memset(buffer, 0xa5, 64);
        emu.set_last_error(MARKER as u64);
        assert_eq!(call(&mut emu, library, legacy, &[handle, buffer]), 1);
        let attributes = emu.maps.read_dword(buffer).unwrap();
        assert_eq!(emu.maps.read_dword(buffer + 32), Some(0));
        assert_eq!(emu.maps.read_dword(buffer + 36), Some(3));
        assert_eq!(emu.maps.read_dword(buffer + 40), Some(1));
        assert_ne!(emu.maps.read_qword(buffer + 44), Some(0));
        assert_eq!(emu.maps.read_dword(buffer + 52), Some(0xa5a5_a5a5));
        assert_eq!(emu.get_last_error(), MARKER as u64);
        let class = FILE_ATTRIBUTE_TAG_INFO_CLASS as u64;
        let tag_size = 2 * size_of::<u32>() as u64;
        emu.maps.memset(buffer, 0xa5, 64);
        assert_eq!(
            call(
                &mut emu,
                library,
                extended,
                &[handle, class, buffer, tag_size - 1]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_BAD_LENGTH);
        assert_eq!(emu.maps.read_dword(buffer), Some(0xa5a5_a5a5));
        assert_eq!(
            call(
                &mut emu,
                library,
                extended,
                &[handle, class, buffer, tag_size + 8]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(buffer), Some(attributes));
        assert_eq!(emu.maps.read_dword(buffer + 4), Some(0));
        assert_eq!(emu.maps.read_dword(buffer + tag_size), Some(0xa5a5_a5a5));
        assert_eq!(
            call(
                &mut emu,
                library,
                extended,
                &[u32::MAX as u64, class, buffer, tag_size]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        let last_write = 132_400_000_000_000_000;
        emu.maps.write_qword(output, last_write);
        emu.set_last_error(MARKER as u64);
        assert_eq!(
            call(&mut emu, library, "SetFileTime", &[handle, 0, 0, output]),
            1
        );
        emu.maps.write_qword(output, 0);
        assert_eq!(
            call(&mut emu, library, "GetFileTime", &[handle, 0, 0, output]),
            1
        );
        assert_eq!(emu.maps.read_qword(output), Some(last_write));
        assert_eq!(emu.get_last_error(), MARKER as u64);
        // Validate every output before writing the first FILETIME.
        emu.maps.write_qword(output, 99);
        assert_eq!(
            call(&mut emu, library, "GetFileTime", &[handle, output, 1, 0]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
        assert_eq!(emu.maps.read_qword(output), Some(99));
        emu.maps.write_qword(output, i64::MIN as u64);
        assert_eq!(
            call(&mut emu, library, "SetFileTime", &[handle, 0, 0, output]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
        assert_eq!(
            call(&mut emu, library, "SetFileTime", &[handle, 0, 0, 1]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
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
            call(&mut emu, library, "SetFileTime", &[read_only, 0, 0, 0]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
        let metadata_only = call(
            &mut emu,
            library,
            create,
            &[
                data,
                FILE_WRITE_ATTRIBUTES as u64,
                (FILE_SHARE_READ | FILE_SHARE_WRITE) as u64,
                0,
                OPEN_EXISTING as u64,
                0,
                0,
            ],
        );
        assert_ne!(metadata_only as u32, u32::MAX);
        emu.maps.write_qword(output, last_write);
        assert_eq!(
            call(
                &mut emu,
                library,
                "SetFileTime",
                &[metadata_only, 0, 0, output]
            ),
            1
        );
        assert_eq!(call(&mut emu, library, "CloseHandle", &[metadata_only]), 1);
        for api in ["GetFileTime", "SetFileTime"] {
            assert_eq!(call(&mut emu, library, api, &[u32::MAX as u64, 0, 0, 0]), 0);
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        }
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
fn file_changes_are_visible_through_both_libraries() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let drive = file_system_root().join("c");
        fs::create_dir_all(&drive).unwrap();
        let name = format!(
            "File-Changes-{}-{library}-{is64}-{wide}",
            std::process::id()
        );
        let temp = tempfile::Builder::new()
            .prefix(&name)
            .rand_bytes(0)
            .tempdir_in(&drive)
            .unwrap();
        let (mut emu, data) = prepare(is64);
        emu.cfg.cwd_path = format!(r"C:\{}", temp.path().file_name().unwrap().to_str().unwrap());
        let src = data + 256;
        let dst = data + 512;
        let moved = data + 768;
        let suffix = if wide { "W" } else { "A" };
        for (address, path) in [
            (data, "sub"),
            (src, "source"),
            (dst, r"sub\copy"),
            (moved, r"sub\moved"),
        ] {
            if wide {
                emu.maps.write_wide_string(address, path);
            } else {
                emu.maps.write_string(address, path);
            }
        }
        let mut invoke =
            |name: &str, args: &[u64]| call(&mut emu, library, &format!("{name}{suffix}"), args);
        fs::write(temp.path().join("Source"), b"content").unwrap();
        assert_eq!(invoke("CreateDirectory", &[data, 0]), 1);
        assert_eq!(invoke("CreateDirectory", &[data, 0]), 0);
        assert_eq!(invoke("CopyFile", &[src, dst, 1]), 1);
        assert_eq!(invoke("CopyFile", &[src, dst, 1]), 0);
        assert_eq!(invoke("DeleteFile", &[src]), 1);
        assert!(!temp.path().join("Source").exists());
        assert_eq!(invoke("MoveFile", &[dst, moved]), 1);
        assert_eq!(fs::read(temp.path().join("sub/moved")).unwrap(), b"content");
        assert_eq!(invoke("RemoveDirectory", &[data]), 0);
        assert_eq!(invoke("DeleteFile", &[moved]), 1);
        assert_eq!(invoke("DeleteFile", &[moved]), 0);
        assert_eq!(invoke("RemoveDirectory", &[data]), 1);
        assert!(!temp.path().join("sub").exists());
    }
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
    fs::create_dir_all(file_system_root().join("c/guest-data")).unwrap();
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

#[test]
fn paired_dos_device_exports_return_encoded_multisz() {
    for (is64, library, (name, wide)) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [("QueryDosDeviceA", false), ("QueryDosDeviceW", true)]
    ) {
        let (mut emu, data) = prepare(is64);
        emu.maps.memset(data, 0xa5, 64);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, name, &[0, data, 3]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(emu.maps.read_byte(data), Some(0xa5));
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, name, &[0, data, 4]),
            4
        );
        assert_eq!(
            if wide {
                emu.maps.read_wide_string(data)
            } else {
                emu.maps.read_string(data)
            },
            "C:"
        );
        let width = if wide { 2 } else { 1 };
        assert_eq!(emu.maps.read_byte(data + 3 * width), Some(0));
        assert_eq!(emu.maps.read_byte(data + 4 * width), Some(0xa5));
    }
}

#[test]
fn suffix_array_exports_return_a_pointer_into_the_original_string() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let array = data + 128;
        let wrong = data + 160;
        let matching = data + 192;
        let stride = pointer_size(&emu);
        assert!(write_pointer(&mut emu, array, wrong));
        assert!(write_pointer(&mut emu, array + stride, matching));
        for (name, wide) in [
            ("PathFindSuffixArrayA", false),
            ("PathFindSuffixArrayW", true),
        ] {
            let source_prefix = if wide {
                emu.maps.write_wide_string(data, "🪶sample.txt");
                emu.maps.write_wide_string(wrong, ".TXT");
                emu.maps.write_wide_string(matching, ".txt");
                "🪶sample".encode_utf16().count() * size_of::<u16>()
            } else {
                emu.maps.write_bytes(data, b"\xe9sample.txt\0");
                emu.maps.write_string(wrong, ".TXT");
                emu.maps.write_string(matching, ".txt");
                b"\xe9sample".len()
            };
            for (count, expected) in [(0, 0), (1, 0), (2, data + source_prefix as u64)] {
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        "shlwapi.dll",
                        name,
                        &[data, array, count]
                    ),
                    expected,
                );
            }
        }
    }
}

#[test]
fn paired_content_type_exports_consult_the_guest_registry() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let key = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Classes\\.mwemu_content_type_test";
        emu.registry.create_key(key).unwrap();
        assert!(
            emu.registry
                .set_value(key, "Content Type", registry_store::sz("text/plain"))
        );
        for (name, wide) in [("PathIsContentTypeA", false), ("PathIsContentTypeW", true)] {
            if wide {
                emu.maps
                    .write_wide_string(data, "file.mwemu_content_type_test");
                emu.maps.write_wide_string(data + 128, "TEXT/PLAIN");
            } else {
                emu.maps.write_string(data, "file.mwemu_content_type_test");
                emu.maps.write_string(data + 128, "TEXT/PLAIN");
            }
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "shlwapi.dll", name, &[data, data + 128]),
                1
            );
            if wide {
                emu.maps
                    .write_wide_string(data + 128, "application/octet-stream");
            } else {
                emu.maps
                    .write_string(data + 128, "application/octet-stream");
            }
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "shlwapi.dll", name, &[data, data + 128]),
                0
            );
        }
        assert!(emu.registry.delete_key(key, false).is_ok());
    }
}

#[test]
fn device_io_control_checks_handles_and_preserves_unknown_control_outputs() {
    for (wide, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, data) = prepare(wide);
        let console = call(
            &mut emu,
            library,
            "GetStdHandle",
            &[STD_OUTPUT_HANDLE as u64],
        );
        let returned = data + 0x100;
        let mut args = [console, 0, data, 16, data + 0x200, 32, returned, 0];
        emu.set_api_mode(ApiMode::SimulateFail);
        emu.maps.write_dword(returned, MARKER);
        assert_eq!(call(&mut emu, library, "DeviceIoControl", &args), 0);
        assert_eq!(emu.get_last_error(), ERROR_CALL_NOT_IMPLEMENTED);
        assert_eq!(emu.maps.read_dword(returned), Some(MARKER));
        args[0] = 0;
        assert_eq!(call(&mut emu, library, "DeviceIoControl", &args), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        args[0] = console;
        args[6] = 1;
        assert_eq!(call(&mut emu, library, "DeviceIoControl", &args), 0);
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
    }
}

#[test]
fn shell_folder_queries_share_encodings_architectures_and_dll_routes() {
    const MARKER: u8 = 0xcc;
    for is64 in [false, true] {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        for (library, wide, (folder, expected)) in iproduct!(
            ["shell32.dll", "shfolder.dll"],
            [false, true],
            [
                (CSIDL_WINDOWS, r"C:\Windows"),
                (CSIDL_MYPICTURES, r"C:\Users\User\Pictures"),
                (CSIDL_PROFILE, r"C:\Users\User"),
                (CSIDL_PROGRAM_FILES_COMMON, r"C:\Program Files\Common Files")
            ]
        ) {
            for flags in [0, CSIDL_FLAG_CREATE | CSIDL_FLAG_DONT_VERIFY] {
                assert!(emu.maps.write_bytes(output, &[MARKER; MAX_PATH * 2 + 1]));
                emu.set_last_error(MARKER as u64);
                let api = if wide {
                    "SHGetFolderPathW"
                } else {
                    "SHGetFolderPathA"
                };
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        api,
                        &[
                            0,
                            (folder | flags) as u64,
                            0,
                            SHGFP_TYPE_CURRENT as u64,
                            output
                        ]
                    ),
                    S_OK,
                    "{library}!{api}, is64={is64}, folder={folder}"
                );
                assert_eq!(strings::read_text(&emu, output, wide), expected);
                assert_eq!(emu.get_last_error(), MARKER as u64);
                let end = (expected.len() + 1) * if wide { 2 } else { 1 };
                assert_eq!(emu.maps.read_byte(output + end as u64), Some(MARKER));
            }
        }
    }
}

#[test]
fn shell_folder_queries_clear_failed_outputs_and_keep_boolean_results() {
    for (is64, wide) in iproduct!([false, true], [false, true]) {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        let api = if wide {
            "SHGetFolderPathW"
        } else {
            "SHGetFolderPathA"
        };
        for (folder, token, kind, expected) in [
            (CSIDL_WINDOWS, 0, 2, HRESULT_E_INVALID_ARG),
            (CSIDL_WINDOWS, 123, SHGFP_TYPE_CURRENT, E_NOTIMPL),
            (CSIDL_CONTROLS, 0, SHGFP_TYPE_CURRENT, S_FALSE),
            (0x7f, 0, SHGFP_TYPE_CURRENT, HRESULT_E_INVALID_ARG),
        ] {
            assert!(emu.maps.write_bytes(output, &[0xcc; MAX_PATH * 2]));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "shell32.dll",
                    api,
                    &[0, folder as u64, token, kind as u64, output]
                ),
                expected
            );
            assert_eq!(strings::read_text(&emu, output, wide), "");
            assert_eq!(
                emu.maps.read_byte(output + if wide { 2 } else { 1 }),
                Some(0xcc)
            );
        }
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "shell32.dll",
                api,
                &[
                    0,
                    CSIDL_PROFILE as u64,
                    u64::MAX,
                    SHGFP_TYPE_DEFAULT as u64,
                    output
                ]
            ),
            S_OK
        );
        assert_eq!(strings::read_text(&emu, output, wide), r"C:\Users\Default");
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "shell32.dll",
                api,
                &[0, CSIDL_WINDOWS as u64, 0, 0, 0]
            ),
            E_POINTER
        );
        let special = if wide {
            "SHGetSpecialFolderPathW"
        } else {
            "SHGetSpecialFolderPathA"
        };
        for (folder, result) in [(CSIDL_WINDOWS, 1), (CSIDL_CONTROLS, 0), (0x7f, 0)] {
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "shell32.dll",
                    special,
                    &[0, output, folder as u64, 0]
                ),
                result
            );
        }
    }
}

const RESOURCE_RVA: u64 = 0x1000;
const RESOURCE_DATA_ENTRY: u64 = 0x60;
const RESOURCE_PAYLOAD: u64 = 0x2100;
const RESOURCE_TYPE_ID: u64 = 10;
const RESOURCE_NAME_ID: u64 = 300;
const RESOURCE_SUBDIRECTORY: u32 = 1 << 31;

fn prepare_resource_image(is64: bool, named: bool) -> (Emu, u64, u64) {
    let (mut emu, data) = prepare(is64);
    let base = if is64 { 0x1_4000_0000 } else { 0x400000 };
    emu.base = base;
    let original = if is64 {
        helpers::LOADER64
    } else {
        helpers::LOADER32
    };
    let mut image = vec![0; 0x3000];
    image[..0x400].copy_from_slice(&original[..0x400]);
    let nt = u32::from_le_bytes(image[0x3c..0x40].try_into().unwrap()) as usize;
    let optional = nt + 24;
    let directory = optional + if is64 { 112 } else { 96 };
    image[directory..directory + 16 * 8].fill(0);
    for (offset, value) in [
        (optional + 56, image.len() as u32),
        (directory + 2 * 8, RESOURCE_RVA as u32),
        (directory + 2 * 8 + 4, 0xc0),
    ] {
        image[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    emu.maps
        .create_map(
            "resource_image",
            base,
            image.len() as u64,
            Permission::READ_WRITE,
        )
        .unwrap();
    emu.maps.write_bytes(base, &image);
    for (offset, id, target) in [
        (0, RESOURCE_TYPE_ID, 0x20),
        (0x20, RESOURCE_NAME_ID, 0x40),
        (0x40, EN_US_LOCALE as u64, RESOURCE_DATA_ENTRY),
    ] {
        let address = base + RESOURCE_RVA + offset;
        let is_name = named && offset < 0x40;
        emu.maps
            .write_word(address + if is_name { 12 } else { 14 }, 1);
        emu.maps.write_dword(
            address + 16,
            if is_name {
                RESOURCE_SUBDIRECTORY | (0x80 + offset) as u32
            } else {
                id as u32
            },
        );
        emu.maps.write_dword(
            address + 20,
            target as u32
                | if offset < 0x40 {
                    RESOURCE_SUBDIRECTORY
                } else {
                    0
                },
        );
    }
    for (offset, name) in [(0x80, "CUSTOMTYPE"), (0xa0, "CUSTOMNAME")] {
        let address = base + RESOURCE_RVA + offset;
        emu.maps.write_word(address, name.len() as u16);
        for (index, unit) in name.encode_utf16().enumerate() {
            emu.maps.write_word(address + 2 + index as u64 * 2, unit);
        }
    }
    emu.maps.write_dword(
        base + RESOURCE_RVA + RESOURCE_DATA_ENTRY,
        RESOURCE_PAYLOAD as u32,
    );
    emu.maps
        .write_dword(base + RESOURCE_RVA + RESOURCE_DATA_ENTRY + 4, 4);
    emu.maps.write_bytes(base + RESOURCE_PAYLOAD, b"DATA");
    (emu, base, data)
}

#[test]
fn activation_context_rejects_invalid_requests_and_missing_module_manifests() {
    let flags = ACTCTX_FLAG_HMODULE_VALID
        | ACTCTX_FLAG_SET_PROCESS_DEFAULT
        | ACTCTX_FLAG_RESOURCE_NAME_VALID
        | ACTCTX_FLAG_LANGID_VALID;
    for (is64, (dll, api)) in iproduct!(
        [false, true],
        [
            ("kernel32.dll", "CreateActCtxA"),
            ("kernel32.dll", "CreateActCtxW"),
            ("kernelbase.dll", "CreateActCtxW")
        ]
    ) {
        let (mut emu, base, actctx) = prepare_resource_image(is64, false);
        let (size, resource, module) = if is64 { (56, 32, 48) } else { (32, 20, 28) };
        let invalid_handle = if is64 {
            INVALID_HANDLE_VALUE_64
        } else {
            INVALID_HANDLE_VALUE_32
        };
        let source = actctx + 128;
        if api.ends_with('W') {
            emu.maps.write_wide_string(source, r"C:\fixture.exe");
        } else {
            emu.maps.write_string(source, r"C:\fixture.exe");
        }
        write_pointer(&mut emu, actctx + 8, source);
        write_pointer(&mut emu, actctx + module, base);
        for (size, flags, id, error) in [
            (size, flags, 0, ERROR_INVALID_PARAMETER),
            (size - 1, flags, 1, ERROR_INVALID_PARAMETER),
            (size, flags | 0x100, 1, ERROR_INVALID_PARAMETER),
            (size, flags, 1, ERROR_RESOURCE_TYPE_NOT_FOUND),
        ] {
            emu.maps.write_dword(actctx, size);
            emu.maps.write_dword(actctx + 4, flags);
            write_pointer(&mut emu, actctx + resource, id);
            assert_eq!(call(&mut emu, dll, api, &[actctx]), invalid_handle);
            assert_eq!(emu.get_last_error(), error);
        }
        emu.maps.write_dword(base + RESOURCE_RVA + 16, RT_MANIFEST);
        assert_eq!(call(&mut emu, dll, api, &[actctx]), invalid_handle);
        assert_eq!(emu.get_last_error(), ERROR_RESOURCE_NAME_NOT_FOUND);
    }
}

#[test]
fn resource_apis_share_loaded_image_metadata_and_payload_addresses() {
    for (is64, named) in iproduct!([false, true], [false, true]) {
        let (mut emu, base, data) = prepare_resource_image(is64, named);
        let expected = base + RESOURCE_RVA + RESOURCE_DATA_ENTRY;
        for (library, wide) in iproduct!(["kernel32.dll", "kernelbase.dll"], [false, true]) {
            let write_name = |emu: &mut Emu, address, text: &str| {
                let bytes = strings::encode_text(&format!("{text}\0"), wide);
                assert!(emu.maps.write_bytes(address, &bytes));
            };
            write_name(&mut emu, data, if named { "customname" } else { "#300" });
            write_name(
                &mut emu,
                data + 0x100,
                if named { "CustomType" } else { "#10" },
            );
            let mut keys = vec![(data, data + 0x100)];
            if !named {
                keys.push((RESOURCE_NAME_ID, RESOURCE_TYPE_ID));
            }
            for (name, kind) in keys {
                emu.set_last_error(MARKER as u64);
                assert_eq!(
                    call(
                        &mut emu,
                        library,
                        if wide {
                            "FindResourceW"
                        } else {
                            "FindResourceA"
                        },
                        &[0, name, kind]
                    ),
                    expected
                );
                for language in [0, EN_US_LOCALE as u64] {
                    assert_eq!(
                        call(
                            &mut emu,
                            library,
                            if wide {
                                "FindResourceExW"
                            } else {
                                "FindResourceExA"
                            },
                            &[base, kind, name, language]
                        ),
                        expected
                    );
                }
                assert_eq!(
                    call(&mut emu, library, "SizeofResource", &[base, expected]),
                    4
                );
                let loaded = call(&mut emu, library, "LoadResource", &[base, expected]);
                assert_eq!(loaded, base + RESOURCE_PAYLOAD);
                assert_eq!(call(&mut emu, library, "LockResource", &[loaded]), loaded);
                assert_eq!(
                    emu.maps.read_bytes_option(loaded, 4).unwrap().as_ref(),
                    b"DATA"
                );
                assert_eq!(call(&mut emu, library, "FreeResource", &[loaded]), 0);
                assert_eq!(
                    call(&mut emu, library, "SizeofResource", &[base, expected]),
                    4
                );
                assert_eq!(emu.get_last_error(), MARKER as u64);
            }
        }
        // An explicit module handle selects that image rather than emu.base.
        let other = base + 0x10000;
        let image = emu
            .maps
            .read_bytes_option(base, 0x3000)
            .unwrap()
            .into_owned();
        emu.maps
            .create_map("resource_dll", other, 0x3000, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_bytes(other, &image);
        let name = if named { data } else { RESOURCE_NAME_ID };
        let kind = if named {
            data + 0x100
        } else {
            RESOURCE_TYPE_ID
        };
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "FindResourceW",
                &[other, name, kind]
            ),
            other + RESOURCE_RVA + RESOURCE_DATA_ENTRY
        );
    }
}

#[test]
fn resource_apis_reject_missing_keys_invalid_ranges_and_fabricated_handles() {
    for is64 in [false, true] {
        let (mut emu, base, data) = prepare_resource_image(is64, false);
        let resource = base + RESOURCE_RVA + RESOURCE_DATA_ENTRY;
        let sentinel = emu
            .maps
            .read_bytes_option(base, 0x3000)
            .unwrap()
            .into_owned();
        for library in ["kernel32.dll", "kernelbase.dll"] {
            for (name, kind, language, error) in [
                (RESOURCE_NAME_ID, 99, 0, ERROR_RESOURCE_TYPE_NOT_FOUND),
                (99, RESOURCE_TYPE_ID, 0, ERROR_RESOURCE_NAME_NOT_FOUND),
                (
                    RESOURCE_NAME_ID,
                    RESOURCE_TYPE_ID,
                    1031,
                    ERROR_RESOURCE_LANG_NOT_FOUND,
                ),
                (data + 0x1000, RESOURCE_TYPE_ID, 0, ERROR_NOACCESS),
            ] {
                assert_eq!(
                    call(
                        &mut emu,
                        library,
                        "FindResourceExW",
                        &[base, kind, name, language]
                    ),
                    0
                );
                assert_eq!(emu.get_last_error(), error);
            }
            for api in ["LoadResource", "SizeofResource"] {
                assert_eq!(call(&mut emu, library, api, &[base, base - 4]), 0);
                assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
            }
            for api in ["FindResourceA", "FindResourceW"] {
                assert_eq!(
                    call(
                        &mut emu,
                        library,
                        api,
                        &[data, RESOURCE_NAME_ID, RESOURCE_TYPE_ID]
                    ),
                    0
                );
                assert_eq!(emu.get_last_error(), ERROR_BAD_EXE_FORMAT);
            }
            assert_eq!(call(&mut emu, library, "LockResource", &[0]), 0);
            assert_eq!(
                emu.maps.read_bytes_option(base, 0x3000).unwrap().as_ref(),
                sentinel
            );
            emu.maps.write_dword(resource, u32::MAX);
            assert_eq!(
                call(
                    &mut emu,
                    library,
                    "FindResourceA",
                    &[base, RESOURCE_NAME_ID, RESOURCE_TYPE_ID]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_BAD_EXE_FORMAT);
            assert_eq!(
                call(&mut emu, library, "LoadResource", &[base, resource]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_BAD_EXE_FORMAT);
            emu.maps.write_dword(resource, RESOURCE_PAYLOAD as u32);
        }
    }
}

#[test]
fn version_queries_return_views_in_the_callers_resource_block() {
    use crate::windows::structures::{VS_FIXEDFILEINFO, VS_VERSIONINFO};
    use minidump::format::VS_FFI_SIGNATURE;

    fn node(key: &str, text: bool, value: &[u8], children: &[Vec<u8>]) -> Vec<u8> {
        let mut bytes = vec![0; 6];
        let count = if text { value.len() / 2 } else { value.len() };
        bytes[2..4].copy_from_slice(&(count as u16).to_le_bytes());
        bytes[4..6].copy_from_slice(&(text as u16).to_le_bytes());
        bytes.extend(strings::encode_text(&format!("{key}\0"), true));
        bytes.resize(bytes.len().next_multiple_of(4), 0);
        bytes.extend_from_slice(value);
        for child in children {
            bytes.resize(bytes.len().next_multiple_of(4), 0);
            bytes.extend_from_slice(child);
        }
        let len = bytes.len() as u16;
        bytes[..2].copy_from_slice(&len.to_le_bytes());
        bytes
    }

    let title = "Café";
    let company = node(
        "CompanyName",
        true,
        &strings::encode_text(&format!("{title}\0"), true),
        &[],
    );
    let table = node("040904e4", true, &[], &[company]);
    let names = node("StringFileInfo", true, &[], &[table]);
    let mut fixed = vec![0; VS_FIXEDFILEINFO::size()];
    fixed[..4].copy_from_slice(&VS_FFI_SIGNATURE.to_le_bytes());
    let root = node(
        VS_VERSIONINFO::KEY,
        false,
        &fixed,
        std::slice::from_ref(&names),
    );

    for is64 in [false, true] {
        let (mut emu, base, data) = prepare_resource_image(is64, false);
        emu.cfg.exe_name = "sample.exe".into();
        emu.maps.write_dword(base + RESOURCE_RVA + 16, RT_VERSION);
        emu.maps.write_dword(base + RESOURCE_RVA + 0x20 + 16, 1);
        emu.maps.write_dword(
            base + RESOURCE_RVA + RESOURCE_DATA_ENTRY + 4,
            root.len() as u32,
        );
        assert!(emu.maps.write_bytes(base + RESOURCE_PAYLOAD, &root));
        // Store the resource directory and payload at different raw offsets from their RVAs.
        let filename = format!("version-resource-{}.dll", if is64 { 64 } else { 32 });
        let file_path = format!("C:\\{filename}");
        let mut file = vec![0u8; 0xc00];
        let headers = emu.maps.read_bytes(base, 0x400);
        file[..headers.len()].copy_from_slice(&headers);
        let pe = u32::from_le_bytes(file[0x3c..0x40].try_into().unwrap()) as usize;
        let optional_size = u16::from_le_bytes(file[pe + 20..pe + 22].try_into().unwrap()) as usize;
        file[pe + 6..pe + 8].copy_from_slice(&2u16.to_le_bytes());
        let first = pe + 24 + optional_size;
        for (index, rva, raw) in [(0, RESOURCE_RVA, 0x400u32), (1, RESOURCE_PAYLOAD, 0x800u32)] {
            let section = &mut file[first + 40 * index..][..40];
            section.fill(0);
            section[..5].copy_from_slice(b".rsrc");
            section[8..12].copy_from_slice(&0x400u32.to_le_bytes());
            section[12..16].copy_from_slice(&(rva as u32).to_le_bytes());
            section[16..20].copy_from_slice(&0x400u32.to_le_bytes());
            section[20..24].copy_from_slice(&raw.to_le_bytes());
        }
        file[0x400..0x4c0].copy_from_slice(&emu.maps.read_bytes(base + RESOURCE_RVA, 0xc0));
        file[0x800..0x800 + root.len()].copy_from_slice(&root);
        let directory = file_system_root().join("c");
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join(&filename), &file).unwrap();
        let mui_dir = directory.join("en-US");
        fs::create_dir_all(&mui_dir).unwrap();
        let mut sidecar = file.clone();
        let pos = sidecar
            .windows(fixed.len())
            .position(|bytes| bytes == fixed)
            .unwrap();
        sidecar[pos + 8..pos + 12].copy_from_slice(&0x12345678_u32.to_le_bytes());
        let mui = mui_dir.join(format!("{filename}.mui"));
        fs::write(&mui, sidecar).unwrap();
        let block = data + 0x400;
        let output = data + 0x200;
        let length = data + 0x210;
        for (library, wide, image) in iproduct!(
            ["version.dll", "kernelbase.dll"],
            [false, true],
            ["sample.exe", file_path.as_str()]
        ) {
            let (size_api, get_api, query_api) = if wide {
                (
                    "GetFileVersionInfoSizeW",
                    "GetFileVersionInfoW",
                    "VerQueryValueW",
                )
            } else {
                (
                    "GetFileVersionInfoSizeA",
                    "GetFileVersionInfoA",
                    "VerQueryValueA",
                )
            };
            assert!(
                emu.maps
                    .write_bytes(data, &strings::encode_text(&format!("{image}\0"), wide))
            );
            emu.maps.write_qword(length, u64::MAX);
            let size_ex = size_api.replace("Size", "SizeEx");
            let get_ex = get_api.replace("Info", "InfoEx");
            for flags in [0, 1, 2] {
                let required = call(&mut emu, library, &size_ex, &[flags, data, length]);
                assert!(required >= root.len() as u64);
                assert_eq!(
                    call(
                        &mut emu,
                        library,
                        &get_ex,
                        &[flags, data, 0, required, block]
                    ),
                    1
                );
                let offset = (6 + ((VS_VERSIONINFO::KEY.encode_utf16().count() + 1) * 2))
                    .next_multiple_of(4);
                let expected = if flags == 1 && image == file_path {
                    0x12345678
                } else {
                    0
                };
                assert_eq!(
                    emu.maps.read_dword(block + offset as u64 + 8),
                    Some(expected)
                );
            }
            let size = call(&mut emu, library, size_api, &[data, length]);
            assert!(size >= root.len() as u64);
            assert_eq!(emu.maps.read_qword(length), Some(u64::MAX << 32));
            assert_eq!(call(&mut emu, library, get_api, &[data, 0, size, block]), 1);
            for path in ["\\", "\\StringFileInfo\\040904E4\\companyname"] {
                assert!(
                    emu.maps
                        .write_bytes(data, &strings::encode_text(&format!("{path}\0"), wide))
                );
                emu.maps.write_qword(output, u64::MAX);
                assert_eq!(
                    call(&mut emu, library, query_api, &[block, data, output, length]),
                    1
                );
                let ptr = if is64 {
                    emu.maps.read_qword(output).unwrap()
                } else {
                    assert_eq!(emu.maps.read_dword(output + 4), Some(u32::MAX));
                    emu.maps.read_dword(output).unwrap() as u64
                };
                assert!((block..block + size).contains(&ptr));
                let count = emu.maps.read_dword(length).unwrap();
                if path == "\\" {
                    assert_eq!(emu.maps.read_dword(ptr), Some(VS_FFI_SIGNATURE));
                    assert_eq!(count, fixed.len() as u32);
                } else if wide {
                    assert_eq!(emu.maps.read_wide_string(ptr), title);
                    assert_eq!(count, 5);
                } else {
                    assert_eq!(
                        emu.maps.read_bytes(ptr, count as usize).as_ref(),
                        b"Caf\xe9\0"
                    );
                }
            }
            assert!(
                emu.maps
                    .write_bytes(data, &strings::encode_text("\\missing\0", wide))
            );
            assert_eq!(
                call(&mut emu, library, query_api, &[block, data, output, length]),
                0
            );
            assert_eq!(emu.maps.read_dword(length), Some(0));
            // A declared child cannot extend beyond the caller's version block.
            assert!(
                emu.maps
                    .write_bytes(data, &strings::encode_text("\\StringFileInfo\0", wide))
            );
            let first_child = (root.len() - names.len()) as u64;
            emu.maps.write_word(block + first_child, u16::MAX);
            emu.maps.write_qword(output, u64::MAX);
            assert_eq!(
                call(&mut emu, library, query_api, &[block, data, output, length]),
                0
            );
            assert_eq!(emu.maps.read_qword(output), Some(u64::MAX));

            assert!(
                emu.maps
                    .write_bytes(data, &strings::encode_text(&format!("{image}\0"), wide))
            );
            emu.maps.write_dword(block + 8, MARKER);
            assert_eq!(call(&mut emu, library, get_api, &[data, 0, 8, block]), 1);
            assert_eq!(emu.maps.read_dword(block + 8), Some(MARKER));
            assert!(
                emu.maps
                    .write_bytes(data, &strings::encode_text("C:\\other\\sample.exe\0", wide))
            );
            assert_eq!(call(&mut emu, library, size_api, &[data, 0]), 0);
            assert_eq!(emu.get_last_error(), ERROR_FILE_NOT_FOUND);
        }
        fs::remove_file(mui).unwrap();
    }
}

#[test]
fn filename_code_page_applies_across_dlls_to_paths_and_open_files() {
    fs::create_dir_all(file_system_root().join("c/café")).unwrap();
    let dir = file_system_root().join("c/api-code-page");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("café.txt"), b"file").unwrap();
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let input = data;
        let output = data + 0x200;
        assert_eq!(call(&mut emu, "kernel32.dll", "AreFileApisANSI", &[]), 1);
        for (setter, ansi, accent) in [
            ("SetFileApisToOEM", false, 0x82),
            ("SetFileApisToANSI", true, 0xe9),
        ] {
            emu.set_last_error(MARKER as u64);
            call(&mut emu, "kernelbase.dll", setter, &[]);
            assert_eq!(
                call(&mut emu, "kernel32.dll", "AreFileApisANSI", &[]),
                u64::from(ansi)
            );
            assert_eq!(emu.get_last_error(), MARKER as u64);
            let mut raw = b"C:\\api-code-page\\caf".to_vec();
            raw.push(accent);
            raw.extend_from_slice(b".txt\0");
            emu.maps.write_bytes(input, &raw);
            let h = call(
                &mut emu,
                "kernel32.dll",
                "CreateFileA",
                &[
                    input,
                    GENERIC_READ as u64,
                    FILE_SHARE_READ as u64,
                    0,
                    OPEN_EXISTING as u64,
                    0,
                    0,
                ],
            );
            assert_ne!(h as u32, u32::MAX);
            assert_eq!(call(&mut emu, "kernel32.dll", "GetFileSize", &[h, 0]), 4);
            call(&mut emu, "kernel32.dll", "CloseHandle", &[h]);
            emu.maps.write_wide_string(input, "C:\\café");
            assert_eq!(
                call(&mut emu, "kernel32.dll", "SetCurrentDirectoryW", &[input]),
                1
            );
            assert_eq!(
                call(
                    &mut emu,
                    "kernelbase.dll",
                    "GetCurrentDirectoryA",
                    &[16, output]
                ),
                7
            );
            assert_eq!(emu.maps.read_byte(output + 6), Some(accent));
        }
    }
}

#[test]
fn full_paths_count_encoded_units_and_point_to_the_filename() {
    for (is64, dll, wide) in iproduct!(
        [false, true],
        ["kernelbase.dll", "kernel32.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let output = data + 0x200;
        let part = data + 0x500;
        emu.cfg.cwd_path = "C:\\work".to_string();
        let (api, expected, offset, width) = if wide {
            emu.maps.write_wide_string(data, "old\\..\\😀\\leaf");
            ("GetFullPathNameW", 15, 11, 2)
        } else {
            emu.maps.write_string(data, "old\\..\\sub\\leaf");
            ("GetFullPathNameA", 16, 12, 1)
        };
        emu.maps.memset(output, 0xcc, 64);
        emu.maps.memset(part, 0xcc, 8);
        assert_eq!(
            call(&mut emu, dll, api, &[data, expected, output, part]),
            expected + 1
        );
        assert_eq!(emu.maps.read_dword(output), Some(MARKER));
        assert_eq!(emu.maps.read_dword(part), Some(MARKER));
        assert_eq!(
            call(&mut emu, dll, api, &[data, expected + 1, output, part]),
            expected
        );
        let ptr = read_pointer(&emu, part).unwrap();
        assert_eq!(ptr, output + offset * width);
        assert_eq!(strings::read_text(&emu, ptr, wide), "leaf");
        if !is64 {
            assert_eq!(emu.maps.read_dword(part + 4), Some(MARKER));
        }
    }
}

#[test]
fn extended_open_path_and_length_share_the_file_handle() {
    let dir = file_system_root().join("c/extended-open");
    fs::create_dir_all(&dir).unwrap();
    for (is64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, data) = prepare(is64);
        let output = data + 512;
        let params = data + 256;
        let name = format!("open-{is64}-{library}");
        fs::write(dir.join(&name), b"abc").unwrap();
        emu.cfg.cwd_path = r"C:\extended-open".to_owned();
        emu.maps.write_wide_string(data, &name);
        let open = [
            data,
            u64::from(GENERIC_READ | GENERIC_WRITE),
            0,
            OPEN_ALWAYS as u64,
            params,
        ];
        let invalid_handle = if is64 {
            INVALID_HANDLE_VALUE_64
        } else {
            INVALID_HANDLE_VALUE_32
        };
        emu.maps.write_dword(params, 1);
        assert_eq!(
            call(&mut emu, library, "CreateFile2", &open),
            invalid_handle
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
        emu.maps.write_dword(params, if is64 { 32 } else { 24 });
        let h = call(&mut emu, library, "CreateFile2", &open);
        assert_ne!(h, invalid_handle);
        assert_eq!(emu.get_last_error(), ERROR_ALREADY_EXISTS);
        emu.cfg.cwd_path = r"C:\Windows".to_owned();
        for (api, wide) in [
            ("GetFinalPathNameByHandleA", false),
            ("GetFinalPathNameByHandleW", true),
        ] {
            let expected = format!(r"\\?\C:\extended-open\{name}");
            let needed = call(&mut emu, library, api, &[h, 0, 0, 0]);
            assert_eq!(needed, expected.len() as u64 + 1);
            emu.maps.write_dword(output, MARKER);
            assert_eq!(
                call(&mut emu, library, api, &[h, output, needed - 1, 0]),
                needed
            );
            assert_eq!(emu.maps.read_dword(output), Some(MARKER));
            assert_eq!(
                call(&mut emu, library, api, &[h, output, needed, 0]),
                needed - 1
            );
            let path = if wide {
                emu.maps.read_wide_string(output)
            } else {
                emu.maps.read_string(output)
            };
            assert_eq!(path, expected);
        }
        emu.maps.write_qword(output, 8);
        let set_length = [h, u64::from(FILE_END_OF_FILE_INFO_CLASS), output, 8];
        assert_eq!(
            call(&mut emu, library, "SetFileInformationByHandle", &set_length),
            1
        );
        assert_eq!(call(&mut emu, library, "GetFileSize", &[h, 0]), 8);
        assert_eq!(
            call(
                &mut emu,
                library,
                "SetFilePointer",
                &[h, 0, 0, u64::from(FILE_CURRENT)]
            ),
            0
        );
        assert_eq!(call(&mut emu, library, "CloseHandle", &[h]), 1);
        assert_eq!(
            call(
                &mut emu,
                library,
                "GetFinalPathNameByHandleW",
                &[h, output, 64, 0]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        emu.maps
            .write_wide_string(data, r"C:\extended-open\missing-parent\new.bin");
        assert_eq!(
            call(&mut emu, library, "CreateFile2", &open),
            invalid_handle
        );
        assert_eq!(emu.get_last_error(), ERROR_PATH_NOT_FOUND);
        assert!(!dir.join("missing-parent").exists());
    }
}

#[test]
fn file_search_walks_guest_entries_and_keeps_each_cursor() {
    let dir = file_system_root().join("c/search-family");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("first.bin"), b"one").unwrap();
    fs::write(dir.join("second.bin"), b"twice").unwrap();
    fs::write(dir.join("ignored.txt"), b"").unwrap();
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        emu.cfg.cwd_path = r"C:\search-family".to_owned();
        let out = data + 512;
        let (first, next) = if wide {
            ("FindFirstFileExW", "FindNextFileW")
        } else {
            ("FindFirstFileExA", "FindNextFileA")
        };
        if wide {
            emu.maps.write_wide_string(data, "*.BIN");
        } else {
            emu.maps.write_string(data, "*.BIN");
        }
        let args = [data, 1, out, 0, 0, 0];
        let a = call(&mut emu, library, first, &args);
        let b = call(&mut emu, library, first, &args);
        assert_ne!(a, b);
        let name = |emu: &Emu| {
            if wide {
                emu.maps.read_wide_string(out + 44)
            } else {
                emu.maps.read_string(out + 44)
            }
        };
        assert_eq!(
            (name(&emu), emu.maps.read_dword(out + 32)),
            ("first.bin".to_owned(), Some(3))
        );
        assert_eq!(call(&mut emu, library, next, &[a, 0]), 0);
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
        for h in [a, b] {
            assert_eq!(call(&mut emu, library, next, &[h, out]), 1);
            assert_eq!(
                (name(&emu), emu.maps.read_dword(out + 32)),
                ("second.bin".to_owned(), Some(5))
            );
            assert_eq!(call(&mut emu, library, next, &[h, out]), 0);
            assert_eq!(emu.get_last_error(), ERROR_NO_MORE_FILES);
            assert_eq!(call(&mut emu, library, "FindClose", &[h]), 1);
            assert_eq!(call(&mut emu, library, "FindClose", &[h]), 0);
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        }
    }
}

#[test]
fn disk_and_volume_queries_agree_on_drive_geometry_and_output_width() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let api = |name: &str| format!("{name}{}", if wide { "W" } else { "A" });
        assert_eq!(call(&mut emu, library, "GetLogicalDrives", &[]), 4);
        assert_eq!(
            call(&mut emu, library, &api("GetLogicalDriveStrings"), &[0, 0]),
            5
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                &api("GetLogicalDriveStrings"),
                &[5, data]
            ),
            4
        );
        assert_eq!(call(&mut emu, library, &api("GetDriveType"), &[data]), 3);
        let a = data + 64;
        assert_eq!(
            call(
                &mut emu,
                library,
                &api("GetDiskFreeSpace"),
                &[data, a, a + 8, a + 16, a + 24]
            ),
            1
        );
        let cluster = u64::from(emu.maps.read_dword(a).unwrap())
            * u64::from(emu.maps.read_dword(a + 8).unwrap());
        let free = cluster * u64::from(emu.maps.read_dword(a + 16).unwrap());
        let total = cluster * u64::from(emu.maps.read_dword(a + 24).unwrap());
        assert_eq!(
            call(
                &mut emu,
                library,
                &api("GetDiskFreeSpaceEx"),
                &[data, a, a + 8, a + 16]
            ),
            1
        );
        assert_eq!(
            [a, a + 8, a + 16].map(|p| emu.maps.read_qword(p)),
            [Some(free), Some(total), Some(free)]
        );
        emu.maps.write_dword(a, MARKER);
        assert_eq!(
            call(
                &mut emu,
                library,
                &api("GetVolumeInformation"),
                &[data, 0, 0, a, 0, 0, a + 64, 1]
            ),
            0
        );
        assert_eq!(emu.maps.read_dword(a), Some(MARKER));
        assert_eq!(
            call(
                &mut emu,
                library,
                &api("GetVolumeInformation"),
                &[data, 0, 0, a, 0, 0, a + 64, 5]
            ),
            1
        );
        assert_ne!(emu.maps.read_dword(a), Some(MARKER));
    }
}

#[test]
fn temporary_file_names_and_attributes_refer_to_the_guest_file() {
    let dir = file_system_root().join("c/temp-family");
    fs::create_dir_all(&dir).unwrap();
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let prefix = data + 128;
        let output = data + 256;
        let suffix = if wide { "W" } else { "A" };
        if wide {
            emu.maps.write_wide_string(data, r"C:\temp-family");
            emu.maps.write_wide_string(prefix, "Test");
        } else {
            emu.maps.write_string(data, r"C:\temp-family");
            emu.maps.write_string(prefix, "Test");
        }
        let temp = format!("GetTempFileName{suffix}");
        assert_eq!(
            call(
                &mut emu,
                library,
                &temp,
                &[data, prefix, 0x1234_abcd, output]
            ),
            0xabcd
        );
        let path = if wide {
            emu.maps.read_wide_string(output)
        } else {
            emu.maps.read_string(output)
        };
        assert_eq!(path, r"C:\temp-family\TesABCD.tmp");
        assert!(!dir.join("tesabcd.tmp").exists());
        assert_ne!(
            call(&mut emu, library, &temp, &[data, prefix, 0, output]),
            0
        );
        let set = format!("SetFileAttributes{suffix}");
        let get = format!("GetFileAttributes{suffix}");
        for attr in [FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_NORMAL] {
            assert_eq!(call(&mut emu, library, &set, &[output, attr as u64]), 1);
            let value = call(&mut emu, library, &get, &[output]);
            assert_eq!(
                value & FILE_ATTRIBUTE_READONLY as u64,
                (attr & FILE_ATTRIBUTE_READONLY) as u64
            );
        }
        assert_eq!(
            call(&mut emu, library, &format!("DeleteFile{suffix}"), &[output]),
            1
        );
    }
}

#[test]
fn file_paths_sizes_and_directory_handles_use_existing_entries() {
    let dir = file_system_root().join("c/file-path-family");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("long file name.txt"), b"123456").unwrap();
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let out = data + 256;
        emu.cfg.cwd_path = r"C:\file-path-family".to_owned();
        let text = "long file name.txt";
        if wide {
            emu.maps.write_wide_string(data, text);
        } else {
            emu.maps.write_string(data, text);
        }
        let suffix = if wide { "W" } else { "A" };
        for name in ["GetShortPathName", "GetLongPathName"] {
            let api = format!("{name}{suffix}");
            assert_eq!(
                call(&mut emu, library, &api, &[data, 0, 0]),
                text.len() as u64 + 1
            );
            assert_eq!(
                call(&mut emu, library, &api, &[data, out, 64]),
                text.len() as u64
            );
        }
        emu.maps.write_qword(out, u64::MAX);
        assert_eq!(
            call(
                &mut emu,
                library,
                &format!("GetCompressedFileSize{suffix}"),
                &[data, out]
            ),
            6
        );
        assert_eq!(
            emu.maps.read_qword(out),
            Some(u64::from(u32::MAX) << u32::BITS)
        );
        emu.maps.write_wide_string(data, r"C:\file-path-family");
        let h = call(
            &mut emu,
            library,
            "CreateFileW",
            &[
                data,
                0,
                0,
                0,
                OPEN_EXISTING as u64,
                FILE_FLAG_BACKUP_SEMANTICS as u64,
                0,
            ],
        );
        assert_eq!(call(&mut emu, library, "GetFileType", &[h]), FILE_TYPE_DISK);
        assert_eq!(call(&mut emu, library, "CloseHandle", &[h]), 1);
        assert_eq!(
            call(&mut emu, library, "GetFileType", &[h]),
            FILE_TYPE_UNKNOWN
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
    }
}

#[test]
fn unsupported_file_operations_stop_without_closing_the_input_handle() {
    use crate::winapi::policy::{ApiDisposition, ApiMode, ApiStopReason};
    for (is64, library, (name, slots)) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [
            ("ReadFileEx", 5),
            ("LockFile", 5),
            ("FlushViewOfFile", 2),
            ("LZRead", 3),
            ("Wow64DisableWow64FsRedirection", 1)
        ]
    ) {
        let (mut emu, _) = prepare(is64);
        emu.set_api_mode(ApiMode::Strict);
        let handle = helper::handler_create("file-policy://caller-owned");
        let invoke = |emu: &mut Emu| helpers::invoke_winapi_export(emu, library, name);
        if is64 {
            let mut args = vec![0; slots];
            args[0] = handle;
            helpers::call_winapi64(&mut emu, invoke, &args);
        } else {
            let mut args = vec![0; slots];
            args[0] = handle as u32;
            helpers::call_winapi32(&mut emu, invoke, &args);
        }
        let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
        assert_eq!(
            outcome.disposition,
            ApiDisposition::Stopped(ApiStopReason::StrictPolicy)
        );
        assert_eq!(outcome.call.name, name);
        assert_eq!(outcome.call.library, library.trim_end_matches(".dll"));
        assert!(helper::handler_exist(handle));
        helper::handler_close(handle);
    }
}

#[test]
fn legacy_file_io_shares_contents_position_and_truncation_with_win32() {
    let dir = file_system_root().join("c/legacy-io");
    fs::create_dir_all(&dir).unwrap();
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        emu.cfg.cwd_path = r"C:\legacy-io".to_owned();
        emu.maps.write_string(data, &format!("file-{is64}.txt"));
        let dll = "kernel32.dll";
        let h = call(&mut emu, dll, "_lcreat", &[data, 0]);
        assert_ne!(h, u64::from(u32::MAX));
        let buffer = data + 128;
        emu.maps.write_string(buffer, "abcde");
        assert_eq!(call(&mut emu, dll, "_lwrite", &[h, buffer, 5]), 5);
        assert_eq!(call(&mut emu, dll, "_llseek", &[h, 3, 0]), 3);
        assert_eq!(call(&mut emu, dll, "_hwrite", &[h, 0, 0]), 0);
        assert_eq!(call(&mut emu, "kernelbase.dll", "GetFileSize", &[h, 0]), 3);
        assert_eq!(call(&mut emu, dll, "_lclose", &[h]), 0);
        let h = call(&mut emu, dll, "_lopen", &[data, 0]);
        assert_eq!(call(&mut emu, dll, "_hread", &[h, buffer, 8]), 3);
        assert_eq!(call(&mut emu, dll, "_lread", &[h, buffer, 8]), 0);
        assert_eq!(call(&mut emu, dll, "_lclose", &[h]), 0);
        assert_eq!(call(&mut emu, dll, "_lclose", &[h]), u64::from(u32::MAX));
    }
}

#[test]
fn filesystem_image_queries_use_guest_images_and_machine_types() {
    use goblin::pe::header::{COFF_MACHINE_X86, COFF_MACHINE_X86_64};
    let dir = file_system_root().join("c/image-kind");
    fs::create_dir_all(&dir).unwrap();
    for (name, data) in [
        ("a.exe", helpers::LOADER32),
        ("b.exe", helpers::LOADER64),
        ("plain", &b"plain"[..]),
    ] {
        fs::write(dir.join(name), data).unwrap();
    }
    for (is64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, p) = prepare(is64);
        let out = p + 256;
        emu.cfg.cwd_path = r"C:\image-kind".to_owned();
        for (file, kind) in [("a.exe", Some(0)), ("b.exe", Some(6)), ("plain", None)] {
            emu.maps.write_wide_string(p, file);
            emu.maps.write_dword(out, MARKER);
            assert_eq!(
                call(&mut emu, library, "GetBinaryTypeW", &[p, out]),
                u64::from(kind.is_some())
            );
            assert_eq!(emu.maps.read_dword(out), Some(kind.unwrap_or(MARKER)));
        }
        assert_ne!(
            call(
                &mut emu,
                library,
                "GetSystemWow64Directory2W",
                &[out, 128, u64::from(COFF_MACHINE_X86)]
            ),
            0
        );
        assert!(emu.maps.read_wide_string(out).ends_with(r"\SysWOW64"));
        assert_eq!(
            call(
                &mut emu,
                library,
                "GetSystemWow64Directory2W",
                &[out, 128, u64::from(COFF_MACHINE_X86_64)]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_BAD_ARGUMENTS);
    }
}

#[test]
fn explicit_file_search_returns_the_first_match_and_filename_pointer() {
    let dir = file_system_root().join("c/search-list");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("tool.exe"), b"fixture").unwrap();
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let out = data + 256;
        let part = data + 600;
        let (name, extension) = (data + 128, data + 160);
        for (p, s) in [
            (data, r"C:\missing;C:\search-list"),
            (name, "tool"),
            (extension, ".exe"),
        ] {
            if wide {
                emu.maps.write_wide_string(p, s);
            } else {
                emu.maps.write_string(p, s);
            }
        }
        let api = if wide { "SearchPathW" } else { "SearchPathA" };
        let expected = r"C:\search-list\tool.exe";
        assert_eq!(
            call(
                &mut emu,
                library,
                api,
                &[data, name, extension, 128, out, part]
            ),
            expected.len() as u64
        );
        let ptr = if is64 {
            emu.maps.read_qword(part).unwrap()
        } else {
            u64::from(emu.maps.read_dword(part).unwrap())
        };
        assert_eq!(
            ptr,
            out + r"C:\search-list\".len() as u64 * if wide { 2 } else { 1 }
        );
        let leaf = if wide {
            emu.maps.read_wide_string(ptr)
        } else {
            emu.maps.read_string(ptr)
        };
        assert_eq!(leaf, "tool.exe");
    }
}

#[test]
fn pipe_endpoints_preserve_queue_direction_and_broken_pipe() {
    use crate::winapi::common::memory::read_pointer;
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let first = data;
        let second = data + 8;
        let buf = data + 32;
        let count = data + 64;
        let k = "kernel32.dll";
        let kb = "kernelbase.dll";
        assert_eq!(call(&mut emu, k, "CreatePipe", &[first, second, 0, 0]), 1);
        let reader = read_pointer(&emu, first).unwrap();
        let writer = read_pointer(&emu, second).unwrap();
        assert_ne!(reader, writer);
        assert_eq!(call(&mut emu, kb, "GetFileType", &[reader]), FILE_TYPE_PIPE);
        assert!(emu.maps.write_bytes(buf, b"hello"));
        assert_eq!(
            call(&mut emu, kb, "WriteFile", &[writer, buf, 5, count, 0]),
            1
        );
        assert_eq!(
            call(
                &mut emu,
                k,
                "PeekNamedPipe",
                &[reader, buf, 2, count, count + 4, 0]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(count + 4), Some(5));
        assert_eq!(call(&mut emu, k, "CloseHandle", &[writer]), 1);
        assert_eq!(
            call(&mut emu, kb, "ReadFile", &[reader, buf, 3, count, 0]),
            1
        );
        assert_eq!(emu.maps.read_bytes(buf, 3).as_ref(), b"hel");
        assert_eq!(
            call(&mut emu, k, "ReadFile", &[reader, buf, 4, count, 0]),
            1
        );
        assert_eq!(emu.maps.read_bytes(buf, 2).as_ref(), b"lo");
        assert_eq!(
            call(&mut emu, k, "ReadFile", &[reader, buf, 4, count, 0]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_BROKEN_PIPE);
        assert_eq!(call(&mut emu, k, "CreatePipe", &[first, 0, 0, 0]), 0);
        assert_eq!(read_pointer(&emu, first), Some(reader));
    }
}

#[test]
fn local_named_pipe_and_mailslot_clients_open_through_create_file() {
    use crate::windows::constants::security::{GENERIC_READ, GENERIC_WRITE};
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let k = "kernel32.dll";
        let kb = "kernelbase.dll";
        let buf = data + 256;
        let count = buf + 32;
        for mailslot in [false, true] {
            let name = if mailslot {
                r"\\.\mailslot\queue"
            } else {
                r"\\.\pipe\queue"
            };
            emu.maps.write_wide_string(data, name);
            let (api, args) = if mailslot {
                ("CreateMailslotW", vec![data, 8, 0, 0])
            } else {
                (
                    "CreateNamedPipeW",
                    vec![data, PIPE_ACCESS_DUPLEX.into(), 0, 1, 64, 64, 0, 0],
                )
            };
            let server = call(&mut emu, kb, api, &args);
            assert!(server != 0 && server <= u64::from(u32::MAX - 1));
            emu.maps.write_string(data, name);
            let access = if mailslot {
                GENERIC_WRITE
            } else {
                GENERIC_READ | GENERIC_WRITE
            };
            let client = call(
                &mut emu,
                k,
                "CreateFileA",
                &[data, access.into(), 0, 0, OPEN_EXISTING as u64, 0, 0],
            );
            assert!(client != server && client <= u64::from(u32::MAX - 1));
            assert!(emu.maps.write_bytes(buf, b"abc"));
            assert_eq!(
                call(&mut emu, k, "WriteFile", &[client, buf, 3, count, 0]),
                1
            );
            if mailslot {
                assert_eq!(call(&mut emu, k, "SetMailslotInfo", &[server, 13]), 1);
                assert_eq!(
                    call(
                        &mut emu,
                        kb,
                        "GetMailslotInfo",
                        &[server, count, count + 4, count + 8, count + 12]
                    ),
                    1
                );
                assert_eq!(emu.maps.read_dword(count + 4), Some(3));
                assert_eq!(emu.maps.read_dword(count + 8), Some(1));
                assert_eq!(emu.maps.read_dword(count + 12), Some(13));
                assert_eq!(
                    call(&mut emu, k, "ReadFile", &[server, buf, 2, count, 0]),
                    0
                );
            } else {
                assert_eq!(call(&mut emu, k, "ConnectNamedPipe", &[server, 0]), 0);
                assert_eq!(emu.get_last_error(), ERROR_PIPE_CONNECTED);
            }
            let mut restored = crate::serialization::Serialization::deserialize(
                &crate::serialization::Serialization::serialize(&emu),
            );
            assert_eq!(
                call(&mut restored, kb, "ReadFile", &[server, buf, 8, count, 0]),
                1
            );
            assert_eq!(restored.maps.read_bytes(buf, 3).as_ref(), b"abc");
            if !mailslot {
                assert_eq!(call(&mut restored, k, "DisconnectNamedPipe", &[server]), 1);
                assert_eq!(
                    call(&mut restored, k, "WriteFile", &[client, buf, 1, count, 0]),
                    0
                );
            }
            assert_eq!(call(&mut emu, k, "CloseHandle", &[client]), 1);
            assert_eq!(call(&mut emu, k, "CloseHandle", &[server]), 1);
        }
    }
}

#[test]
fn shell_path_views_use_guest_character_offsets_on_both_dlls() {
    use crate::winapi::common::memory::read_pointer;

    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let output = data + 128;
        for dll in ["kernelbase.dll", "shlwapi.dll"] {
            // One CP1252 byte and one unpaired UTF-16 unit each occupy one guest character.
            assert!(
                emu.maps
                    .write_bytes(data, &[0xe9, b'\\', b'a', b'.', b'x', 0])
            );
            assert_eq!(call(&mut emu, dll, "PathFindExtensionA", &[data]), data + 3);
            assert_eq!(call(&mut emu, dll, "PathFindFileNameA", &[data]), data + 2);
            let units = [0xd800u16, 92, 97, 46, 120, 0];
            for (i, c) in units.into_iter().enumerate() {
                assert!(emu.maps.write_word(data + 2 * i as u64, c));
            }
            assert_eq!(call(&mut emu, dll, "PathFindExtensionW", &[data]), data + 6);
            assert_eq!(call(&mut emu, dll, "PathFindFileNameW", &[data]), data + 4);
            for (path, is_root) in [
                (r"C:\", true),
                (r"\\server\share", true),
                (r"\\server\share\", false),
                (r"dir\file", false),
            ] {
                emu.maps.write_wide_string(data, path);
                assert_eq!(
                    call(&mut emu, dll, "PathIsRootW", &[data]),
                    u64::from(is_root)
                );
            }
        }
        emu.maps.write_wide_string(data, "abc.txt");
        assert!(emu.maps.write_qword(output, u64::MAX));
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "PathCchFindExtension",
                &[data, 8, output]
            ),
            S_OK
        );
        assert_eq!(read_pointer(&emu, output), Some(data + 6));
        if !is64 {
            assert_eq!(emu.maps.read_dword(output + 4), Some(u32::MAX));
        }
    }
}

#[test]
fn pathcch_capacity_and_optional_outputs_follow_the_selected_variant() {
    use crate::winapi::common::memory::read_pointer;
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let dll = "kernelbase.dll";
        let end = data + 64;
        let left = data + 80;
        emu.maps.write_wide_string(data, "abc");
        assert!(emu.maps.write_word(data + 8, 0xcccc));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "PathCchAddBackslashEx",
                &[data, 4, end, left]
            ),
            HRESULT_INSUFFICIENT_BUFFER
        );
        assert_eq!(emu.maps.read_wide_string(data), "abc");
        assert_eq!(emu.maps.read_word(data + 8), Some(0xcccc));
        assert_eq!(read_pointer(&emu, end), Some(0));
        assert_eq!(read_pointer(&emu, left), Some(0));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "PathCchAddBackslashEx",
                &[data, 5, end, left]
            ),
            S_OK
        );
        assert_eq!(read_pointer(&emu, end), Some(data + 8));
        assert_eq!(read_pointer(&emu, left), Some(1));
        assert_eq!(
            call(&mut emu, dll, "PathCchAddBackslash", &[data, 5]),
            S_FALSE
        );
        assert_eq!(
            call(
                &mut emu,
                dll,
                "PathCchRemoveBackslashEx",
                &[data, 5, end, left]
            ),
            S_OK
        );
        assert_eq!(read_pointer(&emu, end), Some(data + 6));
        emu.maps.write_wide_string(data, r"C:\");
        assert_eq!(
            call(
                &mut emu,
                dll,
                "PathCchRemoveBackslashEx",
                &[data, 4, end, left]
            ),
            S_FALSE
        );
        assert_eq!(read_pointer(&emu, end), Some(data + 4));
        assert_eq!(emu.maps.read_wide_string(data), r"C:\");
    }
}

#[test]
fn shell_path_composition_supports_overlapping_input_and_output() {
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let more = data + 100;
        emu.maps.write_wide_string(data, r"C:\base");
        emu.maps.write_wide_string(more, r"dir\..\file");
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "PathCchCombine",
                &[data, 64, data, more]
            ),
            S_OK
        );
        assert_eq!(emu.maps.read_wide_string(data), r"C:\base\file");
        emu.maps.write_string(data, r"C:\base");
        emu.maps.write_string(more, r"\other");
        assert_eq!(
            call(&mut emu, "shlwapi.dll", "PathCombineA", &[data, data, more]),
            data
        );
        assert_eq!(emu.maps.read_string(data), r"C:\other");
        emu.maps.write_wide_string(data, r"C:\a.");
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "PathCchCanonicalizeEx",
                &[data, 64, data, 0]
            ),
            S_OK
        );
        assert_eq!(emu.maps.read_wide_string(data), r"C:\a");
    }
}

#[test]
fn standard_console_modes_share_the_output_buffer_and_validate_handles() {
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let mut std =
            |selector: u32| call(&mut emu, "kernel32.dll", "GetStdHandle", &[selector as u64]);
        let input = std(STD_INPUT_HANDLE);
        let output = std(STD_OUTPUT_HANDLE);
        let err = std(STD_ERROR_HANDLE);
        assert_eq!(
            call(&mut emu, "kernelbase.dll", "SetConsoleMode", &[output, 1]),
            1
        );
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GetConsoleMode", &[err, data]),
            1
        );
        assert_eq!(emu.maps.read_dword(data), Some(1));
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GetConsoleMode", &[input, data]),
            1
        );
        assert_ne!(emu.maps.read_dword(data), Some(1));
        assert_eq!(
            call(&mut emu, "kernelbase.dll", "GetConsoleMode", &[0, data]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
    }
}

#[test]
fn native_writes_validate_buffers_and_report_bytes_at_the_guest_pointer_width() {
    use crate::winapi::common::memory::{pointer_size, read_pointer};
    use crate::windows::constants::errors::{
        STATUS_ACCESS_VIOLATION, STATUS_INVALID_HANDLE, STATUS_SUCCESS,
    };
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let width = pointer_size(&emu);
        let io = data + 16;
        let stdout = call(
            &mut emu,
            "kernel32.dll",
            "GetStdHandle",
            &[STD_OUTPUT_HANDLE as u64],
        );
        let boundary = data + 0x10000;
        for (index, label) in ["nt_write_first", "nt_write_second"]
            .into_iter()
            .enumerate()
        {
            emu.maps
                .create_map(label, boundary + index as u64, 1, Permission::READ_WRITE)
                .unwrap();
        }
        assert!(emu.maps.write_bytes(boundary, b"ok"));
        for (handle, buf, size, expected, bytes) in [
            (stdout, boundary, 2, STATUS_SUCCESS, 2),
            (stdout, 0, 2, STATUS_ACCESS_VIOLATION, 0),
            (0, data, 0, STATUS_INVALID_HANDLE, 0),
        ] {
            emu.maps.write_bytes(io, &[0xa5; 17]);
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "ZwWriteFile",
                    &[handle, 0, 0, 0, io, buf, size, 0, 0]
                ),
                expected
            );
            assert_eq!(read_pointer(&emu, io), Some(expected));
            assert_eq!(read_pointer(&emu, io + width), Some(bytes));
            assert_eq!(emu.maps.read_byte(io + 2 * width), Some(0xa5));
        }
    }
}

#[test]
fn native_path_keeps_dos_devices_and_verbatim_files_in_their_namespaces() {
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let width = pointer_size(&emu);
        let (descriptor, part) = (data + 256, data + 320);
        for (input, expected, file_part) in [
            ("NUL", r"\??\NUL", None),
            (r"C:\NuL:", r"\??\NuL", None),
            ("COM1", r"\??\COM1", None),
            ("LPT²", r"\??\LPT²", None),
            (r"\\.\NUL", r"\??\NUL", Some("NUL")),
            (r"\\?\C:\NUL", r"\??\C:\NUL", Some("NUL")),
            (
                r"\\server\share\NUL",
                r"\??\UNC\server\share\NUL",
                Some("NUL"),
            ),
            (r"C:\COM10", r"\??\C:\COM10", Some("COM10")),
        ] {
            emu.maps.write_wide_string(data, input);
            write_pointer(&mut emu, part, u64::from(MARKER));
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "RtlDosPathNameToNtPathName_U",
                    &[data, descriptor, part, 0]
                ),
                1,
                "{input}"
            );
            let buffer = read_pointer(&emu, descriptor + width).unwrap();
            let part_address = read_pointer(&emu, part).unwrap();
            assert_eq!(emu.maps.read_wide_string(buffer), expected, "{input}");
            assert_eq!(
                (part_address != 0)
                    .then(|| emu.maps.read_wide_string(part_address))
                    .as_deref(),
                file_part,
                "{input}"
            );
            call(&mut emu, "ntdll.dll", "RtlFreeUnicodeString", &[descriptor]);
        }
        emu.maps
            .write_wide_string(data, r"C:\missing-device-parent\NUL");
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "RtlDosPathNameToNtPathName_U",
                &[data, descriptor, part, 0]
            ),
            0
        );
    }
}

#[test]
fn native_null_device_reads_eof_and_reports_a_character_device() {
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let width = pointer_size(&emu);
        let (desc, obj, out, io, buf) =
            (data + 128, data + 192, data + 256, data + 288, data + 320);
        for name in [r"\??\NUL", r"\Device\Null", r"\??\C:\NUL"] {
            emu.maps.write_wide_string(data, name);
            let byte_len = (name.encode_utf16().count() * 2) as u16;
            emu.maps.write_word(desc, byte_len);
            emu.maps.write_word(desc + 2, byte_len + 2);
            write_pointer(&mut emu, desc + width, data);
            emu.maps.write_dword(obj, (6 * width) as u32);
            write_pointer(&mut emu, obj + 2 * width, desc);
            emu.maps.write_dword(obj + 3 * width, OBJ_OPENIF);
            write_pointer(&mut emu, io + width, 99);
            let status = call(
                &mut emu,
                "ntdll.dll",
                "NtCreateFile",
                &[
                    out,
                    u64::from(FILE_GENERIC_READ | GENERIC_WRITE),
                    obj,
                    io,
                    0,
                    0,
                    u64::from(FILE_SHARE_READ),
                    u64::from(FILE_OPEN),
                    u64::from(FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT),
                    0,
                    0,
                ],
            );
            if name == r"\??\C:\NUL" {
                assert_eq!(status, STATUS_OBJECT_NAME_NOT_FOUND);
                assert_eq!(read_pointer(&emu, io + width), Some(99));
                continue;
            }
            assert_eq!(status, STATUS_SUCCESS);
            assert_eq!(read_pointer(&emu, io + width), Some(0));
            let file = read_pointer(&emu, out).unwrap();
            assert_eq!(
                call(&mut emu, "kernel32.dll", "GetFileType", &[file]),
                FILE_TYPE_CHAR
            );
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "NtReadFile",
                    &[file, 0, 0, 0, io, buf, 8, 0, 0]
                ),
                STATUS_END_OF_FILE
            );
            assert_eq!(read_pointer(&emu, io), Some(STATUS_END_OF_FILE));
            assert_eq!(read_pointer(&emu, io + width), Some(0));
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "NtWriteFile",
                    &[file, 0, 0, 0, io, buf, 8, 0, 0]
                ),
                STATUS_SUCCESS
            );
            assert_eq!(read_pointer(&emu, io + width), Some(8));
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtClose", &[file]),
                STATUS_SUCCESS
            );
        }
    }
}

#[test]
fn native_path_open_read_and_close_share_the_guest_file() {
    let root = file_system_root().join("c/native-path-io");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("test.bin"), b"guest").unwrap();
    for is64 in [false, true] {
        let (mut emu, data) = prepare(is64);
        let width = pointer_size(&emu);
        let (desc, part, obj, out, io, buf) = (
            data + 128,
            data + 160,
            data + 192,
            data + 256,
            data + 288,
            data + 320,
        );
        emu.maps
            .write_wide_string(data, r"C:\native-path-io\test.bin");
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "RtlDosPathNameToNtPathName_U",
                &[data, desc, part, 0]
            ),
            1
        );
        assert_eq!(
            emu.maps.read_wide_string(read_pointer(&emu, part).unwrap()),
            "test.bin"
        );
        emu.maps.write_dword(obj, (6 * width) as u32);
        write_pointer(&mut emu, obj + 2 * width, desc);
        let access = u64::from(FILE_GENERIC_READ);
        let options = u64::from(FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT);
        let share = u64::from(FILE_SHARE_READ);
        let open = [out, access, obj, io, share, options];
        let create = [
            out,
            access,
            obj,
            io,
            0,
            0,
            share,
            u64::from(FILE_OPEN),
            options,
            0,
            0,
        ];
        for (api, args) in [
            ("NtCreateFile", create.as_slice()),
            ("ZwOpenFile", open.as_slice()),
        ] {
            assert_eq!(call(&mut emu, "ntdll.dll", api, args), STATUS_SUCCESS);
            assert_eq!(read_pointer(&emu, io + width), Some(FILE_OPENED));
            let file = read_pointer(&emu, out).unwrap();
            for (expected, size) in [(STATUS_SUCCESS, 5), (STATUS_END_OF_FILE, 0)] {
                assert_eq!(
                    call(
                        &mut emu,
                        "ntdll.dll",
                        "ZwReadFile",
                        &[file, 0, 0, 0, io, buf, 7, 0, 0]
                    ),
                    expected
                );
                assert_eq!(read_pointer(&emu, io + width), Some(size));
                for (i, byte) in b"guest".iter().enumerate() {
                    assert_eq!(emu.maps.read_byte(buf + i as u64), Some(*byte));
                }
            }
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtClose", &[file]),
                STATUS_SUCCESS
            );
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtClose", &[file]),
                STATUS_INVALID_HANDLE
            );
        }
        let native = read_pointer(&emu, desc + width).unwrap();
        emu.maps
            .write_wide_string(native, r"\??\C:\native-path-io\none.bin");
        write_pointer(&mut emu, io, 0xf00d);
        write_pointer(&mut emu, io + width, 0xfeed);
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtOpenFile", &open),
            STATUS_OBJECT_NAME_NOT_FOUND
        );
        assert_eq!(read_pointer(&emu, io), Some(0xf00d));
        assert_eq!(read_pointer(&emu, io + width), Some(0xfeed));
        call(&mut emu, "ntdll.dll", "RtlFreeUnicodeString", &[desc]);
        assert_eq!(read_pointer(&emu, desc + width), Some(0));
    }
}

#[test]
fn kernel_console_cursor_round_trips_and_rejects_invalid_updates() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, out) = prepare_winapi(is64);
        let dll = "kernel32.dll";
        let console = call(
            &mut emu,
            dll,
            "GetStdHandle",
            &[u64::from(STD_OUTPUT_HANDLE)],
        );
        emu.maps.write_dword(out, 75);
        emu.maps.write_dword(out + 4, 0);
        assert_eq!(
            call(&mut emu, dll, "SetConsoleCursorInfo", &[console, out]),
            1
        );
        emu.maps.write_bytes(out, &[0xa5; 9]);
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "GetConsoleCursorInfo",
                &[console, out]
            ),
            1
        );
        assert_eq!(emu.maps.read_qword(out), Some(75));
        assert_eq!(emu.maps.read_byte(out + 8), Some(0xa5));
        emu.maps.write_dword(out, 101);
        assert_eq!(
            call(&mut emu, dll, "SetConsoleCursorInfo", &[console, out]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
        assert_eq!(call(&mut emu, dll, "GetConsoleCursorInfo", &[0, out]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        assert_eq!(
            call(&mut emu, dll, "GetConsoleCursorInfo", &[console, out]),
            1
        );
        assert_eq!(emu.maps.read_qword(out), Some(75));
    }
}

#[test]
fn string_resources_bound_copies_and_borrow_guest_utf16() {
    for is64 in [false, true] {
        let (mut emu, base, out) = prepare_resource_image(is64, false);
        emu.maps.write_dword(base + RESOURCE_RVA + 16, RT_STRING);
        emu.maps.write_dword(base + RESOURCE_RVA + 0x20 + 16, 1);
        emu.maps
            .write_dword(base + RESOURCE_RVA + RESOURCE_DATA_ENTRY + 4, 38);
        let payload = base + RESOURCE_PAYLOAD;
        emu.maps.memset(payload, 0, 38);
        emu.maps.write_word(payload + 2, 3);
        emu.maps.write_bytes(payload + 4, b"A\0\0\0Z\0");
        for library in ["kernelbase.dll", "user32.dll"] {
            for api in ["LoadStringA", "LoadStringW"] {
                emu.maps.memset(out, 0xa5, 16);
                assert_eq!(call(&mut emu, library, api, &[base, 1, out, 2]), 1);
                let width = if api.ends_with('W') { 2 } else { 1 };
                assert_eq!(emu.maps.read_byte(out), Some(b'A'));
                assert_eq!(emu.maps.read_byte(out + width), Some(0));
                assert_eq!(emu.maps.read_byte(out + 2 * width), Some(0xa5));
            }
            assert_eq!(
                call(&mut emu, library, "LoadStringW", &[base, 1, out, 0]),
                3
            );
            let pointer = if is64 {
                emu.maps.read_qword(out)
            } else {
                emu.maps.read_dword(out).map(u64::from)
            };
            assert_eq!(pointer, Some(payload + 4));
        }
    }
}

#[test]
fn console_writes_use_console_handles_and_count_input_units() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::{
        errors::ERROR_INVALID_HANDLE, files::STD_OUTPUT_HANDLE, nls::CP_IBM437,
    };
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let handle = call(
            &mut emu,
            "kernel32.dll",
            "GetStdHandle",
            &[u64::from(STD_OUTPUT_HANDLE)],
        );
        for lib in ["kernel32.dll", "kernelbase.dll"] {
            assert_eq!(
                call(&mut emu, lib, "GetConsoleOutputCP", &[]),
                u64::from(CP_IBM437)
            );
            for api in ["WriteConsoleA", "WriteConsoleW"] {
                if api.ends_with('W') {
                    emu.maps.write_word(data, 0x00e9);
                } else {
                    emu.maps.write_byte(data, 0x82);
                }
                emu.maps.write_qword(data + 8, u64::MAX);
                let original = emu.emulated_stdout.len();
                assert_eq!(call(&mut emu, lib, api, &[handle, data, 1, data + 8, 0]), 1);
                assert_eq!(&emu.emulated_stdout[original..], "é".as_bytes());
                assert_eq!(emu.maps.read_dword(data + 8), Some(1));
                assert_eq!(emu.maps.read_dword(data + 12), Some(u32::MAX));
                assert_eq!(call(&mut emu, lib, api, &[0, data, 1, data + 8, 0]), 0);
                assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
            }
        }
    }
}

#[test]
fn shell_string_exports_share_character_width_compare_and_reverse_search() {
    use crate::tests::helpers::{call_winapi_export, prepare_winapi};
    use crate::winapi::common::strings::encode_text;
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let needle = data + 0x80;
        for (suffix, wide, unit) in [("A", false, 1), ("W", true, 2)] {
            emu.maps.write_bytes(data, &encode_text("abABa\0", wide));
            emu.maps.write_bytes(needle, &encode_text("AB\0", wide));
            for dll in ["shell32.dll", "shlwapi.dll", "kernelbase.dll"] {
                let call = |emu: &mut crate::emu::Emu, stem: &str, args: &[u64]| {
                    call_winapi_export(emu, dll, &format!("{stem}{suffix}"), args)
                };
                assert_eq!(
                    call(&mut emu, "StrChr", &[data, b'A' as u64]),
                    data + 2 * unit
                );
                assert_eq!(
                    call(&mut emu, "StrRChrI", &[data, data + 4 * unit, b'A' as u64]),
                    data + 2 * unit
                );
                assert_eq!(
                    call(&mut emu, "StrRStrI", &[data, data + unit, needle]),
                    data
                );
                assert_eq!(call(&mut emu, "StrCmpNI", &[data, needle, 2]), 0);
                assert_eq!(
                    call(&mut emu, "StrCmpN", &[data, needle, 2]),
                    u32::MAX as u64
                );
            }
        }
    }
}
