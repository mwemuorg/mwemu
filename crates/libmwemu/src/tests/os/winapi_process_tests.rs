use crate::windows::peb::{peb32, peb64};
use crate::windows::structures::{
    RtlUserProcessParameters32, RtlUserProcessParameters64, UnicodeString64,
};
use goblin::pe::dll_characteristic::IMAGE_DLLCHARACTERISTICS_NX_COMPAT;
use itertools::iproduct;
use rs_header::pe::pe32::PE32;

use crate::api::windows::common::memory::{read_pointer, write_pointer};
use crate::emu::{Emu, object_handle::ServiceObject};
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::syscall::windows::{ntapi::ntapi32, syscall64};
use crate::tests::helpers;
use crate::threading::context::ThreadContext;
use crate::winapi::common::strings;
use crate::winapi::winapi64;
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::{
    EXCEPTION_CONTINUE_EXECUTION32, EXCEPTION_CONTINUE_SEARCH, EXCEPTION_EXECUTE_HANDLER,
};
use crate::windows::constants::process::*;
use crate::windows::constants::syscalls::{
    WIN64_NTQUERYINFORMATIONPROCESS, WIN64_NTQUERYINFORMATIONTHREAD,
};
use crate::windows::constants::system::*;
use crate::{emu_aarch64, emu64};

const SYSTEM_BASIC_INFORMATION_SIZE: u64 = 64;

#[test]
fn process_dep_policy_tracks_opt_in_and_permanent_settings() {
    let call = helpers::call_winapi_export;
    let dll = "kernel32.dll";
    let (mut emu, out) = helpers::prepare_winapi(false);
    let query = [NT_CURRENT_PROCESS, out, out + 4];
    assert_eq!(
        call(&mut emu, dll, "GetSystemDEPPolicy", &[]),
        u64::from(DEP_OPT_IN)
    );
    assert_eq!(call(&mut emu, dll, "GetProcessDEPPolicy", &query), 1);
    assert_eq!(emu.maps.read_dword(out), Some(0));
    assert_eq!(emu.maps.read_dword(out + 4), Some(0));

    for (flags, result, error) in [
        (0, 1, ERROR_ENVVAR_NOT_FOUND),
        (
            PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION,
            0,
            ERROR_INVALID_PARAMETER,
        ),
        (4, 0, ERROR_INVALID_PARAMETER),
        (PROCESS_DEP_ENABLE, 1, ERROR_ENVVAR_NOT_FOUND),
        (0, 0, ERROR_ACCESS_DENIED),
        (PROCESS_DEP_ENABLE, 0, ERROR_ACCESS_DENIED),
    ] {
        emu.set_last_error(ERROR_ENVVAR_NOT_FOUND);
        assert_eq!(
            call(&mut emu, dll, "SetProcessDEPPolicy", &[flags.into()]),
            result
        );
        assert_eq!(emu.get_last_error(), error);
    }
    let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
    assert_eq!(call(&mut restored, dll, "GetProcessDEPPolicy", &query), 1);
    assert_eq!(restored.maps.read_dword(out), Some(PROCESS_DEP_ENABLE));
    assert_eq!(restored.maps.read_dword(out + 4), Some(1));
}

#[test]
fn process_dep_policy_checks_image_process_and_outputs() {
    let call = helpers::call_winapi_export;
    let dll = "kernel32.dll";
    let both = PROCESS_DEP_ENABLE | PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION;
    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        if !is64 {
            let mut image = PE32::parse("nx.exe", helpers::LOADER32);
            image.opt.dll_characteristics |= IMAGE_DLLCHARACTERISTICS_NX_COMPAT;
            emu.pe32 = Some(image);
        }
        assert_eq!(
            call(&mut emu, dll, "SetProcessDEPPolicy", &[both.into()]),
            0
        );
        assert_eq!(
            emu.get_last_error(),
            if is64 {
                ERROR_NOT_SUPPORTED
            } else {
                ERROR_ACCESS_DENIED
            }
        );
        let query = [NT_CURRENT_PROCESS, out, out + 4];
        assert_eq!(call(&mut emu, dll, "GetProcessDEPPolicy", &query), 1);
        assert_eq!(emu.maps.read_dword(out), Some(both));
        assert_eq!(emu.maps.read_dword(out + 4), Some(1));

        let pid = call(&mut emu, dll, "GetCurrentProcessId", &[]);
        let handle = call(
            &mut emu,
            dll,
            "OpenProcess",
            &[PROCESS_QUERY_LIMITED_INFORMATION.into(), 0, pid],
        );
        assert_ne!(handle, 0);
        for (handle, error) in [(handle, ERROR_ACCESS_DENIED), (0, ERROR_INVALID_HANDLE)] {
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "GetProcessDEPPolicy",
                    &[handle, out, out + 4]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), error);
        }
    }

    let (mut emu, out) = helpers::prepare_winapi(false);
    assert_eq!(
        call(&mut emu, dll, "SetProcessDEPPolicy", &[both.into()]),
        1
    );
    emu.set_hook_on_exception(|_, _, _| false);
    helpers::call_winapi32(
        &mut emu,
        |emu| helpers::invoke_winapi_export(emu, dll, "GetProcessDEPPolicy"),
        &[NT_CURRENT_PROCESS as u32, out as u32, 0],
    );
    assert_eq!(emu.fault_count, 1);
    assert_eq!(emu.maps.read_dword(out), Some(both));
}

#[test]
fn close_handle_preserves_objects_owned_by_other_subsystems() {
    for wide in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(wide);
        let service = emu
            .handle_management
            .open_service_object(ServiceObject::Manager { access: 0 });
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[service]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        assert!(emu.handle_management.close_service_object(service));
    }
}

#[test]
fn top_level_filter_shares_registration_and_returns_the_guest_disposition() {
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, info) = helpers::prepare_winapi(is64);
        let record = info + 0x100;
        let context = info + 0x200;
        let callback = info + 0x1000;
        emu.maps
            .create_map("filter", callback, 0x1000, Permission::READ_WRITE_EXECUTE)
            .unwrap();
        let width = if is64 { 8 } else { 4 };
        assert!(write_pointer(&mut emu, info, record));
        assert!(write_pointer(&mut emu, info + width, context));
        emu.maps.write_dword(record, STATUS_ILLEGAL_INSTRUCTION);

        // The guest increments a counter at ContextRecord+4 and returns the LONG at ContextRecord.
        let body: &[u8] = if is64 {
            &[0x48, 0x8b, 0x49, 8, 0xff, 0x41, 4, 0x8b, 0x01, 0xc3]
        } else {
            &[
                0x8b, 0x4c, 0x24, 4, 0x8b, 0x49, 4, 0xff, 0x41, 4, 0x8b, 0x01, 0xc2, 4, 0,
            ]
        };
        emu.maps.write_bytes(callback, body);
        let filter = "UnhandledExceptionFilter";
        let set = "SetUnhandledExceptionFilter";
        let execute = u64::from(EXCEPTION_EXECUTE_HANDLER);
        assert_eq!(call(&mut emu, "kernelbase.dll", filter, &[info]), execute);
        assert_eq!(call(&mut emu, "kernel32.dll", set, &[callback]), 0);

        let stack = emu.sp();
        emu.threads.push(ThreadContext::new(17, emu.cfg.arch));
        emu.current_thread_id = 1;
        emu.regs_mut().rsp = stack;
        emu.regs_mut().rip = callback + 64;
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(call(&mut emu, "kernelbase.dll", set, &[callback]), callback);
        for (index, (result, expected)) in [
            (
                EXCEPTION_CONTINUE_EXECUTION32,
                EXCEPTION_CONTINUE_EXECUTION32,
            ),
            (EXCEPTION_EXECUTE_HANDLER, EXCEPTION_EXECUTE_HANDLER),
            (EXCEPTION_CONTINUE_SEARCH, EXCEPTION_EXECUTE_HANDLER),
        ]
        .into_iter()
        .enumerate()
        {
            emu.maps.write_dword(context, result);
            assert_eq!(
                call(&mut emu, "kernel32.dll", filter, &[info]),
                u64::from(expected)
            );
            assert_eq!(emu.maps.read_dword(context + 4), Some(index as u32 + 1));
        }

        let peb = callback + 0x1000;
        emu.maps
            .create_map("peb", peb, 0x1000, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_byte(peb + PEB_BEING_DEBUGGED_OFFSET, 1);
        assert_eq!(
            call(&mut emu, "kernelbase.dll", filter, &[info]),
            u64::from(EXCEPTION_CONTINUE_SEARCH)
        );
        assert_eq!(emu.maps.read_dword(context + 4), Some(3));
        assert_eq!(call(&mut emu, "kernelbase.dll", set, &[0]), callback);
        emu.current_thread_id = 0;
        assert_eq!(emu.uef(), 0);
    }
}

#[test]
fn top_level_filter_recovers_the_active_threads_legacy_registration() {
    use crate::serialization::native::NativeSnapshotV5;
    use crate::threading::context::ArchThreadState;

    let (mut emu, first_filter) = helpers::prepare_winapi(false);
    let worker_filter = first_filter + 64;
    emu.set_uef(first_filter);
    emu.threads.push(ThreadContext::new(17, emu.cfg.arch));
    emu.current_thread_id = 1;
    if let ArchThreadState::X86 { uef, .. } = &mut emu.current_thread_mut().arch {
        *uef = worker_filter;
    }
    let mut legacy = b"MWEMU\0\x05\0".to_vec();
    legacy.extend(bitcode::serialize(&NativeSnapshotV5::from(&emu)).unwrap());
    let mut restored = Serialization::deserialize(&legacy);
    assert_eq!(restored.uef(), worker_filter);
    restored.current_thread_id = 0;
    assert_eq!(restored.uef(), worker_filter);
}

#[test]
fn top_level_filter_uses_the_api_policy_on_unmodeled_architectures() {
    for (library, name) in [
        ("kernel32.dll", "UnhandledExceptionFilter"),
        ("kernelbase.dll", "SetUnhandledExceptionFilter"),
        ("ntdll.dll", "RtlSetUnhandledExceptionFilter"),
    ] {
        let mut emu = emu_aarch64();
        helpers::invoke_winapi_export(&mut emu, library, name);
        assert!(emu.api_stopped(), "{library}!{name}");
    }
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

#[derive(Clone, Copy, Debug)]
enum ProcessQueryEntry {
    Hook(&'static str),
    Syscall,
    Sysenter,
}

const PROCESS_QUERY_ENTRIES: [(bool, ProcessQueryEntry); 6] = [
    (false, ProcessQueryEntry::Hook("NtQueryInformationProcess")),
    (false, ProcessQueryEntry::Hook("ZwQueryInformationProcess")),
    (true, ProcessQueryEntry::Hook("NtQueryInformationProcess")),
    (true, ProcessQueryEntry::Hook("ZwQueryInformationProcess")),
    (true, ProcessQueryEntry::Syscall),
    (false, ProcessQueryEntry::Sysenter),
];

fn process_query(emu: &mut Emu, entry: ProcessQueryEntry, args: [u64; 5]) -> u64 {
    nt_information_query(
        emu,
        entry,
        "NtQueryInformationProcess",
        WIN64_NTQUERYINFORMATIONPROCESS,
        args,
    )
}

fn nt_information_query(
    emu: &mut Emu,
    entry: ProcessQueryEntry,
    service: &str,
    number: u64,
    args: [u64; 5],
) -> u64 {
    let sp = emu.sp();
    match entry {
        ProcessQueryEntry::Hook(name) => {
            return helpers::call_winapi_export(emu, "ntdll.dll", name, &args);
        }
        ProcessQueryEntry::Syscall => {
            emu.regs_mut().rax = number;
            emu.regs_mut().rcx = args[0];
            emu.regs_mut().rdx = args[1];
            emu.regs_mut().r8 = args[2];
            emu.regs_mut().r9 = args[3];
            assert!(emu.maps.write_qword(sp + 0x28, args[4]));
            syscall64::gateway(emu);
        }
        ProcessQueryEntry::Sysenter => {
            const SERVICE: u64 = 0x200;
            emu.syscall_name_by_real.insert(SERVICE, service.into());
            emu.regs_mut().rdx = sp;
            for (index, value) in args.into_iter().enumerate() {
                assert!(
                    emu.maps
                        .write_dword(sp + 8 + index as u64 * 4, value as u32)
                );
            }
            ntapi32::gateway(SERVICE, sp, emu);
        }
    }
    assert_eq!(emu.sp(), sp);
    assert!(!emu.api_policy.stopped());
    emu.regs().rax
}

#[test]
fn process_queries_share_layouts_and_preserve_output_boundaries() {
    const MARKER: u8 = 0xcc;
    const LAST_ERROR: u64 = 0x1234;
    for (is64, entry) in PROCESS_QUERY_ENTRIES {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let width = if is64 { 8 } else { 4 };
        let basic_size = if is64 { 48 } else { 24 };
        let extended_size = if is64 { 64 } else { 32 };
        let output = data + 0x100;
        let returned = data + 0x200;
        let peb = data + 0x1000;
        emu.maps
            .create_map("peb", peb, 0x1000, Permission::READ_WRITE)
            .unwrap();
        let pid = helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetCurrentProcessId", &[]);
        for (class, size, expected_status) in [
            (
                PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION,
                basic_size,
                STATUS_SUCCESS,
            ),
            (
                PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION,
                extended_size,
                STATUS_SUCCESS,
            ),
            (
                PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_PORT,
                width,
                STATUS_SUCCESS,
            ),
            (
                PROCESS_INFORMATION_CLASS_PROCESS_WOW64_INFORMATION,
                width,
                STATUS_SUCCESS,
            ),
            (
                PROCESS_INFORMATION_CLASS_PROCESS_BREAK_ON_TERMINATION,
                4,
                STATUS_SUCCESS,
            ),
            (
                PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_FLAGS,
                4,
                STATUS_SUCCESS,
            ),
            (PROCESS_INFORMATION_CLASS_PROCESS_COOKIE, 4, STATUS_SUCCESS),
            (
                PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_OBJECT_HANDLE,
                width,
                STATUS_PORT_NOT_SET,
            ),
        ] {
            assert!(emu.maps.write_bytes(output - 1, &[MARKER; 80]));
            assert!(emu.maps.write_bytes(returned, &[MARKER; 8]));
            emu.set_last_error(LAST_ERROR);
            assert_eq!(
                process_query(
                    &mut emu,
                    entry,
                    [NT_CURRENT_PROCESS, class, output, size, returned]
                ),
                expected_status,
                "{entry:?}, class={class}, size={size}"
            );
            assert_eq!(emu.maps.read_dword(returned), Some(size as u32));
            assert_eq!(
                emu.maps.read_dword(returned + 4),
                Some(u32::from_le_bytes([MARKER; 4]))
            );
            assert_eq!(emu.maps.read_byte(output - 1), Some(MARKER));
            assert_eq!(emu.maps.read_byte(output + size), Some(MARKER));
            assert_eq!(emu.get_last_error(), LAST_ERROR);
            if class == PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION {
                let basic = output + if size == extended_size { width } else { 0 };
                assert_eq!(emu.maps.read_dword(basic), Some(STATUS_PENDING as u32));
                assert_eq!(read_pointer(&emu, basic + width), Some(peb));
                assert_eq!(read_pointer(&emu, basic + 4 * width), Some(pid));
            } else if class == PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_FLAGS {
                assert_eq!(emu.maps.read_dword(output), Some(1));
            } else if class != PROCESS_INFORMATION_CLASS_PROCESS_COOKIE {
                assert!(
                    emu.maps
                        .read_bytes(output, size as usize)
                        .iter()
                        .all(|b| *b == 0)
                );
            }
        }
    }
}

#[test]
fn process_query_errors_preserve_outputs_and_check_complete_ranges() {
    const MARKER: u8 = 0xcc;
    for (is64, entry) in PROCESS_QUERY_ENTRIES {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let size = if is64 { 48 } else { 24 };
        let output = data + 0x100;
        let returned = data + 0x200;
        let basic = PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION;
        for (handle, class, buffer, length, return_length, status) in [
            (
                NT_CURRENT_PROCESS,
                basic,
                output,
                size - 1,
                returned,
                STATUS_INFO_LENGTH_MISMATCH,
            ),
            (
                NT_CURRENT_PROCESS,
                basic,
                output,
                size + 1,
                returned,
                STATUS_INFO_LENGTH_MISMATCH,
            ),
            (0, basic, output, size, returned, STATUS_INVALID_HANDLE),
            (
                NT_CURRENT_THREAD,
                basic,
                output,
                size,
                returned,
                STATUS_INVALID_HANDLE,
            ),
            (
                NT_CURRENT_PROCESS,
                basic,
                0,
                size,
                returned,
                STATUS_ACCESS_VIOLATION,
            ),
            (
                NT_CURRENT_PROCESS,
                basic,
                output,
                size,
                1,
                STATUS_ACCESS_VIOLATION,
            ),
            (
                NT_CURRENT_PROCESS,
                u32::MAX as u64,
                output,
                size,
                returned,
                STATUS_NOT_IMPLEMENTED,
            ),
        ] {
            assert!(emu.maps.write_bytes(output, &[MARKER; 80]));
            assert!(emu.maps.write_bytes(returned, &[MARKER; 8]));
            assert_eq!(
                process_query(
                    &mut emu,
                    entry,
                    [handle, class, buffer, length, return_length]
                ),
                status,
                "{entry:?}, class={class}, length={length}"
            );
            assert_eq!(emu.maps.read_bytes(output, 80).as_ref(), &[MARKER; 80]);
            assert_eq!(emu.maps.read_bytes(returned, 8).as_ref(), &[MARKER; 8]);
        }
        let hole = data + 0x2000;
        emu.maps
            .create_map("short_query_output", hole, size - 1, Permission::READ_WRITE)
            .unwrap();
        assert_eq!(
            process_query(
                &mut emu,
                entry,
                [NT_CURRENT_PROCESS, basic, hole, size, returned]
            ),
            STATUS_ACCESS_VIOLATION
        );
        assert_eq!(
            emu.maps.read_bytes(hole, size as usize - 1).as_ref(),
            vec![0; size as usize - 1]
        );
    }
}

#[test]
fn toolhelp_lists_share_native_layouts_and_snapshot_cursors() {
    const IMAGE_BASE: u64 = 0x0040_0000;
    const IMAGE_SIZE: u64 = 0x2000;
    for is64 in [false, true] {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        emu.base = IMAGE_BASE;
        emu.cfg.exe_name = "snapshot.exe".to_owned();
        emu.maps
            .create_map(
                "snapshot.exe",
                IMAGE_BASE,
                IMAGE_SIZE,
                Permission::READ_EXECUTE,
            )
            .unwrap();
        let pid = helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetCurrentProcessId", &[]);
        let cases = [
            (
                "Process32First",
                "Process32Next",
                TH32CS_SNAPPROCESS,
                if is64 { 304 } else { 296 },
                false,
            ),
            (
                "Process32FirstW",
                "Process32NextW",
                TH32CS_SNAPPROCESS,
                if is64 { 568 } else { 556 },
                true,
            ),
            (
                "Module32First",
                "Module32Next",
                TH32CS_SNAPMODULE,
                if is64 { 568 } else { 548 },
                false,
            ),
            (
                "Module32FirstW",
                "Module32NextW",
                TH32CS_SNAPMODULE,
                if is64 { 1080 } else { 1064 },
                true,
            ),
            (
                "Thread32First",
                "Thread32Next",
                TH32CS_SNAPTHREAD,
                28,
                false,
            ),
        ];
        for (first, next, flags, size, wide) in cases {
            let handle = helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "CreateToolhelp32Snapshot",
                &[flags as u64, 0],
            );
            assert!(emu.maps.write_bytes(output, &vec![0xcc; size + 16]));
            assert!(emu.maps.write_dword(output, size as u32));
            emu.set_last_error(ERROR_ACCESS_DENIED);
            // Next on a new snapshot starts at its first entry; First rewinds it.
            for api in [next, first] {
                assert_eq!(
                    helpers::call_winapi_export(&mut emu, "kernel32.dll", api, &[handle, output]),
                    1,
                    "{api} is64={is64}"
                );
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                assert_eq!(emu.maps.read_dword(output), Some(size as u32));
            }
            match flags {
                TH32CS_SNAPPROCESS => {
                    assert_eq!(emu.maps.read_dword(output + 8), Some(pid as u32));
                    let name = output + if is64 { 44 } else { 36 };
                    let actual = if wide {
                        emu.maps.read_wide_string(name)
                    } else {
                        emu.maps.read_string(name)
                    };
                    assert_eq!(actual, "snapshot.exe");
                }
                TH32CS_SNAPMODULE => {
                    assert_eq!(emu.maps.read_dword(output + 8), Some(pid as u32));
                    assert_eq!(
                        read_pointer(&emu, output + if is64 { 24 } else { 20 }),
                        Some(IMAGE_BASE)
                    );
                    assert_eq!(
                        emu.maps.read_dword(output + if is64 { 32 } else { 24 }),
                        Some(IMAGE_SIZE as u32)
                    );
                    assert_eq!(
                        read_pointer(&emu, output + if is64 { 40 } else { 28 }),
                        Some(IMAGE_BASE)
                    );
                    let name = output + if is64 { 48 } else { 32 };
                    let actual = if wide {
                        emu.maps.read_wide_string(name)
                    } else {
                        emu.maps.read_string(name)
                    };
                    assert_eq!(actual, "snapshot.exe");
                }
                TH32CS_SNAPTHREAD => {
                    assert_eq!(
                        emu.maps.read_dword(output + 8),
                        Some(emu.current_thread().id as u32)
                    );
                    assert_eq!(emu.maps.read_dword(output + 12), Some(pid as u32));
                }
                _ => unreachable!(),
            }
            assert_eq!(
                emu.maps.read_bytes(output + size as u64, 16).as_ref(),
                [0xcc; 16]
            );
            let mut count = 1;
            while helpers::call_winapi_export(&mut emu, "kernel32.dll", next, &[handle, output])
                != 0
            {
                count += 1;
                assert!(count <= 3, "{next} must terminate");
            }
            assert_eq!(emu.get_last_error(), ERROR_NO_MORE_FILES);
            assert_eq!(count, if flags == TH32CS_SNAPMODULE { 2 } else { 1 });
            let exhausted = emu.maps.read_bytes(output, size + 16).to_vec();
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernel32.dll", next, &[handle, output]),
                0
            );
            assert_eq!(emu.maps.read_bytes(output, size + 16).as_ref(), exhausted);
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[handle]),
                1
            );
            assert_eq!(
                helpers::call_winapi_export(&mut emu, "kernel32.dll", first, &[handle, output]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        }
    }
}

#[test]
fn toolhelp_validates_buffers_flags_and_serializes_independent_cursors() {
    use crate::serialization::Serialization;

    for is64 in [false, true] {
        let (mut emu, output) = helpers::prepare_winapi(is64);
        let size = if is64 { 568 } else { 556 };
        let flags = TH32CS_SNAPPROCESS | TH32CS_SNAPTHREAD;
        let snapshot = helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "CreateToolhelp32Snapshot",
            &[flags as u64, 0],
        );
        for supplied_size in [0, size - 1, size + 1] {
            assert!(emu.maps.write_bytes(output, &vec![0xcc; size + 16]));
            assert!(emu.maps.write_dword(output, supplied_size as u32));
            let before = emu.maps.read_bytes(output, size + 16).to_vec();
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "kernel32.dll",
                    "Process32FirstW",
                    &[snapshot, output]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_BAD_LENGTH);
            assert_eq!(emu.maps.read_bytes(output, size + 16).as_ref(), before);
        }
        let short = output + 0x1000 - 4;
        assert!(emu.maps.write_dword(short, size as u32));
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "Process32NextW",
                &[snapshot, short]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
        assert_eq!(emu.maps.read_dword(short), Some(size as u32));
        assert!(emu.maps.write_dword(output, size as u32));
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "Process32NextW",
                &[snapshot, output]
            ),
            1
        );
        let saved = Serialization::serialize(&emu);
        emu = Serialization::deserialize(&saved);
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "Process32NextW",
                &[snapshot, output]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NO_MORE_FILES);
        assert!(emu.maps.write_dword(output, 28));
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "Thread32Next",
                &[snapshot, output]
            ),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "Module32FirstW",
                &[snapshot, output]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NO_MORE_FILES);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[snapshot]),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[snapshot]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        for (flags, error) in [
            (0, ERROR_INVALID_PARAMETER),
            (TH32CS_SNAPHEAPLIST, ERROR_CALL_NOT_IMPLEMENTED),
        ] {
            let result = helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "CreateToolhelp32Snapshot",
                &[flags as u64, 0],
            );
            assert_eq!(
                result,
                if is64 {
                    INVALID_HANDLE_VALUE_64
                } else {
                    INVALID_HANDLE_VALUE_32
                }
            );
            assert_eq!(emu.get_last_error(), error);
        }
    }
}

#[test]
fn environment_variables_share_routes_widths_and_owned_state() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let name = data;
        let value = data + 0x100;
        let output = data + 0x200;
        let suffix = if wide { "W" } else { "A" };
        let set = format!("SetEnvironmentVariable{suffix}");
        let get = format!("GetEnvironmentVariable{suffix}");
        strings::write_string(&mut emu, name, "MWEMU_ENVIRONMENT", wide);
        strings::write_string(&mut emu, value, "value", wide);
        emu.set_last_error(0x1234);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, &set, &[name, value]),
            1
        );
        strings::write_string(&mut emu, name, "mwemu_environment", wide);
        for capacity in [0, 1, 5, 6] {
            assert!(emu.maps.fill_range(output, 0xcc, 32));
            let result =
                helpers::call_winapi_export(&mut emu, library, &get, &[name, output, capacity]);
            assert_eq!(result, if capacity < 6 { 6 } else { 5 });
            let bytes = if capacity < 6 {
                0
            } else if wide {
                12
            } else {
                6
            };
            assert_eq!(emu.maps.read_byte(output + bytes), Some(0xcc));
            if capacity == 6 {
                assert_eq!(strings::read_text(&emu, output, wide), "value");
            }
            assert_eq!(emu.get_last_error(), 0x1234);
        }
        let (mut independent, other) = helpers::prepare_winapi(is64);
        strings::write_string(&mut independent, other, "MWEMU_ENVIRONMENT", wide);
        assert_eq!(
            helpers::call_winapi_export(&mut independent, library, &get, &[other, 0, 0]),
            0
        );
        assert_eq!(independent.get_last_error(), ERROR_ENVVAR_NOT_FOUND);
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, &get, &[name, 0, 0]),
            6
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, &set, &[name, 0]),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, &get, &[name, 0, 0]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_ENVVAR_NOT_FOUND);
    }
}

#[test]
fn environment_blocks_encoding_expansion_and_release_share_one_model() {
    for (is64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let env_slot = if is64 {
            peb64::init_arguments(&mut emu) + RtlUserProcessParameters64::ENVIRONMENT_OFFSET
        } else {
            peb32::init_arguments(&mut emu) + RtlUserProcessParameters32::ENVIRONMENT_OFFSET
        };
        let read_environment = |emu: &Emu| {
            if is64 {
                emu.maps.read_qword(env_slot)
            } else {
                emu.maps.read_dword(env_slot).map(u64::from)
            }
        };
        let original = read_environment(&emu).unwrap();
        assert!(!emu.maps.read_wide_string(original).is_empty());
        let output = data + 0x200;
        emu.maps.write_wide_string(data, "NAME=café €\0EMPTY=\0\0");
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "SetEnvironmentStringsW", &[data]),
            1
        );
        let process_block = read_environment(&emu).unwrap();
        let expected: Vec<u8> = "NAME=café €\0EMPTY=\0\0"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        assert_eq!(
            emu.maps
                .try_read_bytes(process_block, expected.len())
                .as_deref(),
            Some(expected.as_slice())
        );
        assert!(emu.maps.write_bytes(data, b"name\0"));
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "GetEnvironmentVariableA",
                &[data, output, 7]
            ),
            6
        );
        assert_eq!(emu.maps.read_bytes(output, 7).as_ref(), b"caf\xe9 \x80\0");
        for api in [
            "GetEnvironmentStrings",
            "GetEnvironmentStringsA",
            "GetEnvironmentStringsW",
        ] {
            let block = helpers::call_winapi_export(&mut emu, library, api, &[]);
            assert_ne!(block, 0);
            let wide = api.ends_with('W');
            if wide {
                assert_eq!(emu.maps.read_wide_string(block), "NAME=café €");
            } else {
                assert_eq!(
                    emu.maps.read_bytes(block, 12).as_ref(),
                    b"NAME=caf\xe9 \x80\0"
                );
            }
            let free = if wide {
                "FreeEnvironmentStringsW"
            } else {
                "FreeEnvironmentStringsA"
            };
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, free, &[data]),
                0
            );
            assert!(emu.maps.is_mapped(data));
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, free, &[block]),
                1
            );
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, free, &[block]),
                0
            );
        }
        emu.maps.write_wide_string(data, "%nAmE%/%UNKNOWN%/%%");
        let expected = "café €/%UNKNOWN%/%%";
        let required = expected.encode_utf16().count() as u64 + 1;
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "ExpandEnvironmentStringsW",
                &[data, 0, 0]
            ),
            required
        );
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "ExpandEnvironmentStringsW",
                &[data, output, required]
            ),
            required
        );
        assert_eq!(emu.maps.read_wide_string(output), expected);
        emu.maps
            .write_wide_string(data, "NAME=changed\0MALFORMED\0\0");
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "SetEnvironmentStringsW", &[data]),
            0
        );
        emu.maps.write_string(data, "NAME");
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "GetEnvironmentVariableA",
                &[data, output, 7]
            ),
            6
        );
        assert_eq!(emu.maps.read_bytes(output, 7).as_ref(), b"caf\xe9 \x80\0");
        emu.maps.write_wide_string(data, "SUPPLEMENTARY");
        emu.maps.write_wide_string(data + 0x100, "A😀B");
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "SetEnvironmentVariableW",
                &[data, data + 0x100]
            ),
            1
        );
        emu.maps.write_string(data, "SUPPLEMENTARY");
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "GetEnvironmentVariableA",
                &[data, output, 4]
            ),
            3
        );
        assert_eq!(emu.maps.read_bytes(output, 4).as_ref(), b"A?B\0");
        emu.maps.write_wide_string(data, "\0");
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "SetEnvironmentStringsW", &[data]),
            1
        );
        assert_eq!(
            emu.maps.read_dword(read_environment(&emu).unwrap()),
            Some(0)
        );
    }
}

#[test]
fn environment_queries_validate_complete_output_ranges() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        emu.maps.write_string(data, "LIMITED");
        emu.maps.write_string(data + 0x100, "value");
        helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "SetEnvironmentVariableA",
            &[data, data + 0x100],
        );
        let tail = 0x900000;
        emu.maps
            .create_map("environment_tail", tail, 2, Permission::READ_WRITE)
            .unwrap();
        assert!(emu.maps.write_bytes(tail, &[0xcc, 0xcc]));
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "GetEnvironmentVariableA",
                &[data, tail, 6]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
        assert_eq!(emu.maps.read_bytes(tail, 2).as_ref(), vec![0xcc, 0xcc]);
        emu.maps
            .create_map("environment_tail_rest", tail + 2, 4, Permission::READ_WRITE)
            .unwrap();
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernelbase.dll",
                "GetEnvironmentVariableA",
                &[data, tail, 6]
            ),
            5
        );
        for (index, byte) in b"value\0".iter().enumerate() {
            assert_eq!(emu.maps.read_byte(tail + index as u64), Some(*byte));
        }
    }
}

#[test]
fn access_mask_exports_expand_generic_bits_and_distinguish_all_from_any() {
    use crate::windows::constants::security::*;
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, mask) = helpers::prepare_winapi(is64);
        let mapping = mask + 16;
        for (i, rights) in [1, 2, 4, 7].into_iter().enumerate() {
            assert!(emu.maps.write_dword(mapping + i as u64 * 4, rights));
        }
        for (dll, prefix) in [
            ("advapi32.dll", ""),
            ("kernelbase.dll", ""),
            ("ntdll.dll", "Rtl"),
        ] {
            for (granted, desired, all, any) in
                [(3, 0, 1, 0), (3, 1, 1, 1), (3, 5, 0, 1), (3, 4, 0, 0)]
            {
                assert_eq!(
                    call(
                        &mut emu,
                        dll,
                        &format!("{prefix}AreAllAccessesGranted"),
                        &[granted, desired]
                    ),
                    all
                );
                assert_eq!(
                    call(
                        &mut emu,
                        dll,
                        &format!("{prefix}AreAnyAccessesGranted"),
                        &[granted, desired]
                    ),
                    any
                );
            }
            for (value, expected) in [
                (GENERIC_READ | GENERIC_WRITE | 16, 19),
                (GENERIC_EXECUTE, 4),
                (GENERIC_ALL, 7),
            ] {
                emu.set_last_error(ERROR_INVALID_PARAMETER);
                assert!(emu.maps.write_dword(mask, value));
                call(
                    &mut emu,
                    dll,
                    &format!("{prefix}MapGenericMask"),
                    &[mask, mapping],
                );
                assert_eq!(emu.maps.read_dword(mask), Some(expected));
                assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
            }
        }
    }
}
#[test]
fn acl_exports_report_used_space_and_return_an_entry_in_the_callers_buffer() {
    use crate::winapi::common::memory::read_pointer;
    use crate::windows::constants::security::*;
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, base) = helpers::prepare_winapi(is64);
        let output = base + 0x100;
        for (dll, create, query, ace, native) in [
            (
                "advapi32.dll",
                "InitializeAcl",
                "GetAclInformation",
                "GetAce",
                false,
            ),
            (
                "kernelbase.dll",
                "InitializeAcl",
                "GetAclInformation",
                "GetAce",
                false,
            ),
            (
                "ntdll.dll",
                "RtlCreateAcl",
                "RtlQueryInformationAcl",
                "RtlGetAce",
                true,
            ),
        ] {
            let success = if native { STATUS_SUCCESS } else { 1 };
            emu.set_last_error(ERROR_FILE_NOT_FOUND);
            assert!(emu.maps.write_bytes(base, &[0xcc; 48]));
            assert_eq!(
                call(&mut emu, dll, create, &[base, 43, ACL_REVISION as u64]),
                success
            );
            assert_eq!(
                emu.maps.read_bytes(base, 9).as_ref(),
                &[2, 0, 40, 0, 0, 0, 0, 0, 0xcc]
            );
            assert_eq!(emu.get_last_error(), ERROR_FILE_NOT_FOUND);
            // ACCESS_ALLOWED_ACE with a 12-byte S-1-5-18 SID.
            let entry = [0, 0, 20, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
            assert!(emu.maps.write_bytes(base + 8, &entry));
            assert!(emu.maps.write_word(base + 4, 1));
            assert!(emu.maps.write_bytes(output, &[0xcc; 16]));
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    query,
                    &[base, output, 12, ACL_SIZE_INFORMATION as u64]
                ),
                success
            );
            assert_eq!(emu.maps.read_dword(output), Some(1));
            assert_eq!(emu.maps.read_dword(output + 4), Some(28));
            assert_eq!(emu.maps.read_dword(output + 8), Some(12));
            assert_eq!(emu.maps.read_dword(output + 12), Some(0xcccccccc));
            assert_eq!(call(&mut emu, dll, ace, &[base, 0, output]), success);
            assert_eq!(read_pointer(&emu, output), Some(base + 8));
            assert_eq!(
                call(&mut emu, dll, ace, &[base, 1, output]),
                if native { STATUS_INVALID_PARAMETER } else { 0 }
            );
            if !native {
                assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
            }
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    query,
                    &[base, output, 11, ACL_SIZE_INFORMATION as u64]
                ),
                if native { STATUS_BUFFER_TOO_SMALL } else { 0 }
            );
            assert_eq!(read_pointer(&emu, output), Some(base + 8));
            if !native {
                assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
            }
        }
    }
}

#[test]
fn shell_app_id_is_owned_by_the_process_and_returned_in_task_memory() {
    use crate::maps::buffers::BufferKind;
    use crate::winapi::common::{buffers, memory::read_pointer};
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let out = data + 0x400;
        const GET: &str = "GetCurrentProcessExplicitAppUserModelID";
        const SET: &str = "SetCurrentProcessExplicitAppUserModelID";
        assert_eq!(call(&mut emu, "shell32.dll", GET, &[out]), E_FAIL);
        assert_eq!(read_pointer(&emu, out), Some(0));
        let id = "a".repeat(127);
        emu.maps.write_wide_string(data, &id);
        assert_eq!(call(&mut emu, "shell32.dll", SET, &[data]), S_OK);
        emu.maps.write_wide_string(data, &"a".repeat(128));
        assert_eq!(
            call(&mut emu, "shell32.dll", SET, &[data]),
            HRESULT_E_INVALID_ARG
        );
        assert_eq!(call(&mut emu, "shell32.dll", GET, &[out]), S_OK);
        let app_id = read_pointer(&emu, out).unwrap();
        assert_eq!(emu.maps.read_wide_string(app_id), id);
        assert_eq!(buffers::size(&emu, app_id, BufferKind::Task), Some(256));
        call(&mut emu, "ole32.dll", "CoTaskMemFree", &[app_id]);
        assert_eq!(buffers::size(&emu, app_id, BufferKind::Task), None);
        emu.maps.write_word(data, 0);
        assert_eq!(call(&mut emu, "shell32.dll", SET, &[data]), S_OK);
        assert_eq!(call(&mut emu, "shell32.dll", GET, &[out]), E_FAIL);
    }
}

#[test]
fn process_and_thread_error_modes_survive_dll_and_thread_switches() {
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        for (dll, other) in [
            ("kernel32.dll", "kernelbase.dll"),
            ("kernelbase.dll", "kernel32.dll"),
        ] {
            emu.set_last_error(ERROR_FILE_NOT_FOUND);
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "SetErrorMode",
                    &[SEM_FAILCRITICALERRORS.into()]
                ),
                0
            );
            assert_eq!(
                call(&mut emu, other, "GetErrorMode", &[]),
                u64::from(SEM_FAILCRITICALERRORS)
            );
            assert_eq!(call(&mut emu, dll, "GetThreadErrorMode", &[]), 0);
            assert!(emu.maps.write_qword(out, u64::MAX));
            assert_eq!(
                call(
                    &mut emu,
                    other,
                    "SetThreadErrorMode",
                    &[SEM_NOGPFAULTERRORBOX.into(), out]
                ),
                1
            );
            assert_eq!(emu.maps.read_qword(out), Some(0xffff_ffff_0000_0000));
            assert_eq!(emu.get_last_error(), ERROR_FILE_NOT_FOUND);
            assert_eq!(
                call(
                    &mut emu,
                    dll,
                    "SetThreadErrorMode",
                    &[SEM_NOALIGNMENTFAULTEXCEPT.into(), out]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
            assert_eq!(
                call(
                    &mut emu,
                    other,
                    "SetThreadErrorMode",
                    &[0, u64::from(u32::MAX)]
                ),
                0
            );
            assert_eq!(
                call(&mut emu, dll, "GetThreadErrorMode", &[]),
                u64::from(SEM_NOGPFAULTERRORBOX)
            );
            let second = emu.threads.len();
            emu.threads.push(ThreadContext::new(99, emu.cfg.arch));
            let test_stack = emu.regs().rsp;
            emu.current_thread_id = second;
            emu.regs_mut().rsp = test_stack;
            assert_eq!(call(&mut emu, dll, "GetThreadErrorMode", &[]), 0);
            emu.current_thread_id = 0;
            let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
            assert_eq!(
                call(&mut restored, dll, "GetErrorMode", &[]),
                u64::from(SEM_FAILCRITICALERRORS)
            );
            assert_eq!(
                call(&mut restored, other, "GetThreadErrorMode", &[]),
                u64::from(SEM_NOGPFAULTERRORBOX)
            );
            assert_eq!(call(&mut emu, other, "SetThreadErrorMode", &[0, 0]), 1);
            assert_eq!(
                call(&mut emu, other, "SetErrorMode", &[0]),
                u64::from(SEM_FAILCRITICALERRORS)
            );
        }
        call(
            &mut emu,
            "kernel32.dll",
            "SetErrorMode",
            &[SEM_NOALIGNMENTFAULTEXCEPT.into()],
        );
        call(&mut emu, "kernelbase.dll", "SetErrorMode", &[0]);
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GetErrorMode", &[]),
            u64::from(SEM_NOALIGNMENTFAULTEXCEPT)
        );
    }
}

#[test]
fn process_and_thread_queries_validate_object_kind_and_exit_status() {
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        let pid = call(&mut emu, "kernel32.dll", "GetCurrentProcessId", &[]);
        let process = call(
            &mut emu,
            "kernelbase.dll",
            "OpenProcess",
            &[PROCESS_QUERY_LIMITED_INFORMATION.into(), 0, pid],
        );
        assert_ne!(process, 0);
        for dll in ["kernel32.dll", "kernelbase.dll"] {
            let thread = call(&mut emu, dll, "GetCurrentThread", &[]);
            assert_eq!(call(&mut emu, dll, "GetProcessId", &[process]), pid);
            emu.maps.write_dword(out, 17);
            assert_eq!(
                call(&mut emu, dll, "CheckRemoteDebuggerPresent", &[process, out]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            assert_eq!(emu.maps.read_dword(out), Some(17));
            assert_eq!(call(&mut emu, dll, "IsWow64Process", &[process, out]), 1);
            assert_eq!(call(&mut emu, dll, "GetProcessIdOfThread", &[thread]), pid);
            assert_eq!(
                call(&mut emu, dll, "GetExitCodeProcess", &[process, out]),
                1
            );
            assert_eq!(emu.maps.read_dword(out), Some(STATUS_PENDING as u32));
            emu.current_thread_mut().exit_value = Some(17);
            assert_eq!(call(&mut emu, dll, "GetExitCodeThread", &[thread, out]), 1);
            assert_eq!(emu.maps.read_dword(out), Some(17));
            assert_eq!(call(&mut emu, dll, "GetExitCodeThread", &[process, out]), 0);
            assert_eq!(emu.maps.read_dword(out), Some(17));
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        }
        assert_eq!(call(&mut emu, "kernel32.dll", "CloseHandle", &[process]), 1);
        assert_eq!(
            call(&mut emu, "kernelbase.dll", "GetProcessId", &[process]),
            0
        );
    }
}

#[test]
fn thread_information_uses_the_selected_thread_and_a_dword_return_length() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        for entry in [
            ProcessQueryEntry::Hook("NtQueryInformationThread"),
            ProcessQueryEntry::Hook("ZwQueryInformationThread"),
            if is64 {
                ProcessQueryEntry::Syscall
            } else {
                ProcessQueryEntry::Sysenter
            },
        ] {
            let (mut emu, data) = helpers::prepare_winapi(is64);
            let width = if is64 { 8 } else { 4 };
            let size = 5 * width + 8;
            let length = data + 64;
            let handle = call(&mut emu, "kernel32.dll", "GetCurrentThread", &[]);
            let id = emu.current_thread().id;
            let query = |emu: &mut Emu, args: [u64; 5]| {
                nt_information_query(
                    emu,
                    entry,
                    "NtQueryInformationThread",
                    WIN64_NTQUERYINFORMATIONTHREAD,
                    args,
                )
            };
            assert!(emu.maps.write_qword(length, u64::MAX));
            assert_eq!(
                query(&mut emu, [handle, 0, data, size, length]),
                STATUS_SUCCESS
            );
            assert_eq!(read_pointer(&emu, data + 3 * width), Some(id));
            assert_eq!(
                emu.maps.read_qword(length),
                Some(0xffff_ffff_0000_0000 | size)
            );
            assert_eq!(
                query(&mut emu, [0, 0, data, size, length]),
                STATUS_INVALID_HANDLE
            );
            assert_eq!(
                query(&mut emu, [handle, 0, data, size - 1, length]),
                STATUS_INFO_LENGTH_MISMATCH
            );
            emu.current_thread_mut().windows_start_address = Some(data);
            assert_eq!(
                query(
                    &mut emu,
                    [
                        handle,
                        THREAD_INFORMATION_CLASS_THREAD_QUERY_SET_WIN32_START_ADDRESS,
                        data,
                        width,
                        length
                    ]
                ),
                STATUS_SUCCESS
            );
            assert_eq!(read_pointer(&emu, data), Some(data));
        }
    }
}

#[test]
fn system_exports_preserve_signed_arithmetic_and_probe_whole_guest_ranges() {
    use crate::maps::mem64::Permission;
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let page = data + 0x10000;
        emu.maps
            .create_map("probe_read", page, 0x1000, Permission::READ)
            .unwrap();
        for dll in ["kernel32.dll", "kernelbase.dll"] {
            for (a, b, c, expected) in [
                (3, 1, 2, 2),
                (-3, 1, 2, -2),
                (3, 1, -2, -2),
                (2, 9, 3, 6),
                (1, 1, 0, -1),
                (i32::MAX, 2, 1, -1),
                (i32::MIN, 1, 1, -1),
            ] {
                let args = [a, b, c].map(|v| u64::from(v as u32));
                assert_eq!(call(&mut emu, dll, "MulDiv", &args) as u32 as i32, expected);
            }
            for (name, args, expected) in [
                ("IsBadCodePtr", vec![page], 0),
                ("IsBadCodePtr", vec![0], 1),
                ("IsBadWritePtr", vec![page, 1], 1),
                ("IsBadHugeWritePtr", vec![data, 4], 0),
                ("IsBadReadPtr", vec![page + 0xfff, 2], 1),
                ("IsBadHugeReadPtr", vec![0, 0], 0),
            ] {
                assert_eq!(call(&mut emu, dll, name, &args), expected, "{dll}!{name}");
            }
            for name in [
                "GetActiveProcessorGroupCount",
                "GetMaximumProcessorGroupCount",
            ] {
                assert_eq!(call(&mut emu, dll, name, &[]), 1);
            }
        }
    }
}

#[test]
fn native_module_lookup_uses_counted_names_and_never_loads_a_missing_dll() {
    use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let width = pointer_size(&emu);
        let (name, out) = (data + 64, data + 160);
        let actual = "ntdll.dll";
        emu.maps.write_wide_string(name, "ntdll.dll_ignored");
        let size = (actual.encode_utf16().count() * 2) as u16;
        emu.maps.write_word(data, size);
        emu.maps.write_word(data + 2, size);
        write_pointer(&mut emu, data + width, name);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "LdrGetDllHandleEx",
                &[1, 0, 0, data, out]
            ),
            STATUS_SUCCESS
        );
        let base = read_pointer(&emu, out).unwrap();
        assert_ne!(base, 0);
        assert_eq!(
            call(&mut emu, "ntdll.dll", "LdrLoadDll", &[0, 0, data, out]),
            STATUS_SUCCESS
        );
        assert_eq!(read_pointer(&emu, out), Some(base));
        emu.maps.write_wide_string(name, "ghost.dll");
        write_pointer(&mut emu, out, 0);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "LdrGetDllHandleEx",
                &[1, 0, 0, data, out]
            ),
            STATUS_DLL_NOT_FOUND
        );
        assert_eq!(read_pointer(&emu, out), Some(0));
    }
}

#[test]
fn native_and_win32_heap_enumeration_share_counts_and_pointer_arrays() {
    use crate::winapi::common::memory::{pointer_size, read_pointer};
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        let process = call(&mut emu, "kernel32.dll", "GetProcessHeap", &[]);
        let other = call(&mut emu, "kernel32.dll", "HeapCreate", &[0, 0, 0]);
        assert_ne!(process, other);
        let width = pointer_size(&emu);
        for (dll, api) in [
            ("ntdll.dll", "RtlGetProcessHeaps"),
            ("kernel32.dll", "GetProcessHeaps"),
            ("kernelbase.dll", "GetProcessHeaps"),
        ] {
            emu.maps.write_bytes(out, &[0xa5; 32]);
            assert_eq!(call(&mut emu, dll, api, &[1, out]), 2);
            assert_eq!(emu.maps.read_byte(out), Some(0xa5));
            assert_eq!(call(&mut emu, dll, api, &[3, out]), 2);
            let stored = [
                read_pointer(&emu, out).unwrap(),
                read_pointer(&emu, out + width).unwrap(),
            ];
            assert!(stored.contains(&process));
            assert!(stored.contains(&other));
            assert_eq!(emu.maps.read_byte(out + 2 * width), Some(0xa5));
        }
    }
}

#[test]
fn kernel_priorities_and_luids_belong_to_the_emulated_machine() {
    use crate::serialization::Serialization;
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::process::{
        HIGH_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, NT_CURRENT_PROCESS, NT_CURRENT_THREAD,
    };
    for is64 in [false, true] {
        let (mut emu, out) = prepare_winapi(is64);
        let dll = "kernel32.dll";
        assert_eq!(
            call(&mut emu, dll, "GetPriorityClass", &[NT_CURRENT_PROCESS]),
            u64::from(NORMAL_PRIORITY_CLASS)
        );
        assert_eq!(
            call(
                &mut emu,
                dll,
                "SetPriorityClass",
                &[NT_CURRENT_PROCESS, u64::from(HIGH_PRIORITY_CLASS)]
            ),
            1
        );
        assert_eq!(
            call(&mut emu, dll, "SetThreadPriority", &[NT_CURRENT_THREAD, 2]),
            1
        );
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "AllocateLocallyUniqueId",
                &[out]
            ),
            1
        );
        let first = emu.maps.read_qword(out).unwrap();
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "GetPriorityClass",
                &[NT_CURRENT_PROCESS]
            ),
            u64::from(HIGH_PRIORITY_CLASS)
        );
        assert_eq!(
            call(&mut emu, dll, "GetThreadPriority", &[NT_CURRENT_THREAD]),
            2
        );
        assert_eq!(
            call(&mut emu, "advapi32.dll", "AllocateLocallyUniqueId", &[out]),
            1
        );
        assert_eq!(emu.maps.read_qword(out), Some(first + 1));
    }
}

#[test]
fn service_handles_reopen_installed_names_across_encodings() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::errors::{ERROR_INVALID_HANDLE, ERROR_SERVICE_DOES_NOT_EXIST};

    let dll = "advapi32.dll";
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let manager = call(&mut emu, dll, "OpenSCManagerW", &[0, 0, 2]);
        assert_ne!(manager, 0);
        emu.maps.write_string(data, "Example");
        let binary = data + 64;
        emu.maps.write_string(binary, "example.exe");
        assert_eq!(call(&mut emu, dll, "OpenServiceA", &[manager, data, 0]), 0);
        assert_eq!(emu.get_last_error(), ERROR_SERVICE_DOES_NOT_EXIST);
        let created = call(
            &mut emu,
            dll,
            "CreateServiceA",
            &[manager, data, 0, 0, 0x10, 2, 1, binary, 0, 0, 0, 0, 0],
        );
        assert_ne!(created, 0);
        emu.maps.write_wide_string(data, "EXAMPLE");
        let opened = call(&mut emu, dll, "OpenServiceW", &[manager, data, 0]);
        assert_ne!(opened, 0);
        assert_ne!(created, opened);
        assert_eq!(call(&mut emu, dll, "OpenServiceW", &[created, data, 0]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        for handle in [opened, created, manager] {
            assert_eq!(call(&mut emu, dll, "CloseServiceHandle", &[handle]), 1);
            assert_eq!(call(&mut emu, dll, "CloseServiceHandle", &[handle]), 0);
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        }
    }
}

#[test]
fn system_queries_preserve_output_boundaries_and_report_the_guest_platform() {
    use crate::maps::virtual_memory::{ALLOCATION_GRANULARITY, PAGE_SIZE};
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use rs_header::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386};
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        for library in ["kernel32.dll", "kernelbase.dll"] {
            emu.maps.memset(data, 0xa5, 176);
            assert_eq!(
                call(
                    &mut emu,
                    library,
                    "GetTimeZoneInformationForYear",
                    &[2024, 0, data]
                ),
                1
            );
            assert_eq!(emu.maps.read_byte(data + 171), Some(0));
            assert_eq!(emu.maps.read_byte(data + 172), Some(0xa5));
            emu.maps.write_dword(data, 8);
            assert_eq!(
                call(&mut emu, library, "GetCurrentPackagePath", &[data, 0]),
                ERROR_INVALID_PARAMETER
            );
            assert_eq!(
                call(
                    &mut emu,
                    library,
                    "GetCurrentPackagePath",
                    &[data, data + 4]
                ),
                APPMODEL_ERROR_NO_PACKAGE
            );
            assert_eq!(emu.maps.read_dword(data), Some(8));
            assert_eq!(call(&mut emu, library, "TerminateProcess", &[0, 15]), 0);
            assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
            for api in ["GetNativeSystemInfo", "GetSystemInfo"] {
                emu.maps.memset(data, 0xa5, 64);
                emu.regs_mut().rax = 17;
                emu.set_last_error(31);
                assert_eq!(call(&mut emu, library, api, &[data]), 17);
                assert_eq!(emu.get_last_error(), 31);
                assert_eq!(
                    emu.maps.read_word(data),
                    Some(if is64 || api == "GetNativeSystemInfo" {
                        9
                    } else {
                        0
                    })
                );
                assert_eq!(emu.maps.read_dword(data + 4), Some(PAGE_SIZE as u32));
                assert_eq!(
                    emu.maps.read_dword(data + 8),
                    Some(ALLOCATION_GRANULARITY as u32)
                );
                let n = if is64 { 48 } else { 36 };
                assert_eq!(
                    emu.maps.read_dword(data + n - 8),
                    Some(ALLOCATION_GRANULARITY as u32)
                );
                assert_eq!(emu.maps.read_byte(data + n), Some(0xa5));
            }
            for api in ["IsWow64Process", "CheckRemoteDebuggerPresent"] {
                emu.maps.memset(data, 0xa5, 8);
                assert_eq!(call(&mut emu, library, api, &[NT_CURRENT_PROCESS, data]), 1);
                assert_eq!(
                    emu.maps.read_dword(data),
                    Some(u32::from(!is64 && api == "IsWow64Process"))
                );
                assert_eq!(emu.maps.read_byte(data + 4), Some(0xa5));
            }
            assert_eq!(
                call(
                    &mut emu,
                    library,
                    "IsWow64Process2",
                    &[NT_CURRENT_PROCESS, data, data + 2]
                ),
                1
            );
            assert_eq!(
                emu.maps.read_word(data),
                Some(if is64 { 0 } else { IMAGE_FILE_MACHINE_I386 })
            );
            assert_eq!(emu.maps.read_word(data + 2), Some(IMAGE_FILE_MACHINE_AMD64));
            assert_eq!(emu.maps.read_byte(data + 4), Some(0xa5));
        }
    }
}

#[test]
fn event_log_queries_and_trace_handles_respect_output_widths() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::errors::{ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_HANDLE};
    const CONTEXT: u64 = 0x0123_4567_89ab_cdef;
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let dll = "advapi32.dll";
        emu.maps.write_wide_string(data, "Application");
        let handle = call(&mut emu, dll, "OpenEventLogW", &[0, data]);
        assert_ne!(handle, 0);
        let buffer = data + 64;
        let needed = buffer + 8;
        emu.maps.write_qword(buffer, u64::MAX);
        assert_eq!(
            call(
                &mut emu,
                dll,
                "GetEventLogInformation",
                &[handle, 0, buffer, 0, needed]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert_eq!(emu.maps.read_dword(needed), Some(4));
        assert_eq!(
            call(
                &mut emu,
                dll,
                "GetEventLogInformation",
                &[handle, 0, buffer, 4, needed]
            ),
            1
        );
        assert_eq!(emu.maps.read_qword(buffer), Some(0xffff_ffff_0000_0000));
        assert_eq!(call(&mut emu, dll, "DeregisterEventSource", &[handle]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        assert_eq!(call(&mut emu, dll, "CloseEventLog", &[handle]), 1);
        emu.maps.write_qword(data + 8, CONTEXT);
        call(&mut emu, dll, "GetTraceLoggerHandle", &[data]);
        let value = if is64 {
            emu.regs().rax
        } else {
            emu.regs().get_eax() | (emu.regs().get_edx() << u32::BITS)
        };
        assert_eq!(value, CONTEXT);
    }
}

#[test]
fn process_outputs_match_guest_layout_and_queries_preserve_guards() {
    for (wide, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, data) = helpers::prepare_winapi(wide);
        let record_size = if wide { 104 } else { 68 };
        let ptr_size = if wide { 8 } else { 4 };
        emu.maps.write_dword(data + record_size, u32::MAX);
        helpers::call_winapi_export(&mut emu, library, "GetStartupInfoW", &[data]);
        assert_eq!(emu.maps.read_dword(data), Some(record_size as u32));
        assert_eq!(emu.maps.read_dword(data + record_size), Some(u32::MAX));
        let handle = if wide { u64::MAX } else { u64::from(u32::MAX) };
        emu.maps.fill_range(data, 0xff, 24);
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "GetProcessAffinityMask",
                &[handle, data, data + ptr_size]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(data), Some(15));
        assert_eq!(emu.maps.read_dword(data + ptr_size), Some(15));
        assert_eq!(emu.maps.read_dword(data + 2 * ptr_size), Some(u32::MAX));
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "IsProcessCritical", &[handle, data]),
            1
        );
        assert_eq!(emu.maps.read_dword(data), Some(0));
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "IsEnclaveTypeSupported", &[1]),
            0
        );
    }
}

#[test]
fn deleting_an_ace_moves_the_next_entry_with_its_flags_and_mask() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::winapi::common::memory::read_pointer;
    use crate::windows::constants::security::{ACCESS_DENIED_ACE_TYPE, ACL_REVISION};
    const INHERIT_TO_CHILD_OBJECTS: u64 = 1;
    const WRITE: u64 = 2;
    let library = "advapi32.dll";
    for is64 in [false, true] {
        let (mut emu, acl) = prepare_winapi(is64);
        let sid = acl + 128;
        let entry_out = sid + 32;
        emu.maps
            .write_bytes(sid, &[1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0]);
        let rev = u64::from(ACL_REVISION);
        assert_eq!(call(&mut emu, library, "InitializeAcl", &[acl, 96, rev]), 1);
        assert_eq!(
            call(
                &mut emu,
                library,
                "AddAccessAllowedAce",
                &[acl, rev, 1, sid]
            ),
            1
        );
        assert_eq!(
            call(
                &mut emu,
                library,
                "AddAccessDeniedAceEx",
                &[acl, rev, INHERIT_TO_CHILD_OBJECTS, WRITE, sid]
            ),
            1
        );
        assert_eq!(call(&mut emu, library, "DeleteAce", &[acl, 0]), 1);
        assert_eq!(emu.maps.read_word(acl + 4), Some(1));
        assert_eq!(call(&mut emu, library, "GetAce", &[acl, 0, entry_out]), 1);
        let entry = read_pointer(&emu, entry_out).unwrap();
        assert_eq!(emu.maps.read_byte(entry), Some(ACCESS_DENIED_ACE_TYPE));
        assert_eq!(
            emu.maps.read_byte(entry + 1),
            Some(INHERIT_TO_CHILD_OBJECTS as u8)
        );
        assert_eq!(emu.maps.read_dword(entry + 4), Some(WRITE as u32));
        assert_eq!(call(&mut emu, library, "DeleteAce", &[acl, 1]), 0);
        assert_eq!(call(&mut emu, library, "IsValidAcl", &[acl]), 1);
    }
}

#[test]
fn adjust_token_privileges_checks_rights_and_sizes_an_empty_previous_state() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    use crate::windows::constants::security::{TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY};

    const DLL: &str = "advapi32.dll";
    const API: &str = "AdjustTokenPrivileges";
    const MARKER: u32 = 0xabab_abab;
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        let new_state = data + 16;
        let previous = data + 64;
        let returned = previous + 32;
        assert!(emu.maps.write_dword(new_state, 0));
        assert_eq!(call(&mut emu, DLL, API, &[0, 0, new_state, 0, 0, 0]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        for rights in [
            TOKEN_QUERY,
            TOKEN_ADJUST_PRIVILEGES,
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
        ] {
            assert_eq!(
                call(
                    &mut emu,
                    DLL,
                    "OpenProcessToken",
                    &[NT_CURRENT_PROCESS, u64::from(rights), data]
                ),
                1
            );
            let token = read_pointer(&emu, data).unwrap();
            assert!(emu.maps.write_dword(returned, MARKER));
            let result = call(&mut emu, DLL, API, &[token, 0, new_state, 0, 0, returned]);
            assert_eq!(result, u64::from(rights & TOKEN_ADJUST_PRIVILEGES != 0));
            assert_eq!(
                emu.get_last_error(),
                if result == 0 {
                    ERROR_ACCESS_DENIED
                } else {
                    ERROR_SUCCESS
                }
            );
            assert_eq!(emu.maps.read_dword(returned), Some(MARKER));
            for size in [4, 16] {
                emu.maps.fill_range(previous, 0xab, 20);
                let result = call(
                    &mut emu,
                    DLL,
                    API,
                    &[token, 0, new_state, size, previous, returned],
                );
                let expected = if rights != TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY {
                    ERROR_ACCESS_DENIED
                } else if size < 16 {
                    ERROR_INSUFFICIENT_BUFFER
                } else {
                    ERROR_SUCCESS
                };
                assert_eq!(result, u64::from(expected == ERROR_SUCCESS));
                assert_eq!(emu.get_last_error(), expected);
                assert_eq!(
                    emu.maps.read_dword(returned),
                    Some(if expected == ERROR_ACCESS_DENIED {
                        MARKER
                    } else {
                        16
                    })
                );
                assert_eq!(
                    emu.maps.read_dword(previous),
                    Some(if result == 0 { MARKER } else { 0 })
                );
                assert_eq!(
                    emu.maps.read_dword(previous + 12),
                    Some(if result == 0 { MARKER } else { 0 })
                );
                assert_eq!(emu.maps.read_dword(previous + 16), Some(MARKER));
            }
        }
    }
}

#[test]
fn duplicate_handle_produces_independent_process_handles() {
    for (wide, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, out) = helpers::prepare_winapi(wide);
        let current = if wide {
            NT_CURRENT_PROCESS
        } else {
            u64::from(u32::MAX)
        };
        let pid = helpers::call_winapi_export(&mut emu, library, "GetCurrentProcessId", &[]);
        let duplicate = |emu: &mut Emu, source| {
            let result = helpers::call_winapi_export(
                emu,
                library,
                "DuplicateHandle",
                &[
                    current,
                    source,
                    current,
                    out,
                    0,
                    0,
                    u64::from(DUPLICATE_SAME_ACCESS),
                ],
            );
            let target = if wide {
                emu.maps.read_qword(out).unwrap()
            } else {
                u64::from(emu.maps.read_dword(out).unwrap())
            };
            (result, target)
        };
        let (success, first) = duplicate(&mut emu, current);
        assert_eq!(success, 1);
        assert_ne!(first, current);
        let (success, second) = duplicate(&mut emu, first);
        assert_eq!(success, 1);
        assert_ne!(first, second);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "CloseHandle", &[first]),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetProcessId", &[second]),
            pid
        );
        assert_eq!(duplicate(&mut emu, 0x7fff0000), (0, 0));
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
    }
}

#[test]
fn process_invalid_close_policy_raises_for_stale_handles() {
    use crate::exception::types::ExceptionType;
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let current = if is64 {
            NT_CURRENT_PROCESS
        } else {
            NT_CURRENT_PROCESS as u32 as u64
        };
        let class = PROCESS_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE;
        let returned = data + 16;
        let options = [current, class, data, 4];
        let query = |emu: &mut Emu| {
            emu.maps.write_dword(data, u32::MAX);
            emu.maps.write_dword(returned, u32::MAX);
            assert_eq!(
                call(
                    emu,
                    "ntdll.dll",
                    "NtQueryInformationProcess",
                    &[current, class, data, 4, returned]
                ),
                STATUS_SUCCESS
            );
            assert_eq!(emu.maps.read_dword(returned), Some(u32::MAX));
            emu.maps.read_dword(data).unwrap()
        };
        assert_eq!(query(&mut emu), 0);
        let event = call(&mut emu, "kernel32.dll", "CreateEventW", &[0, 0, 0, 0]);
        assert_ne!(event, 0);
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[event]),
            STATUS_SUCCESS
        );
        let initial_faults = emu.fault_count;
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[event]),
            STATUS_INVALID_HANDLE
        );
        assert_eq!(emu.fault_count, initial_faults);
        for (value, expected) in [(3, 0), (1, 1), (2, 1)] {
            emu.maps.write_dword(data, value);
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtSetInformationProcess", &options),
                STATUS_SUCCESS
            );
            assert_eq!(query(&mut emu), expected);
        }
        let pid = call(&mut emu, "kernel32.dll", "GetCurrentProcessId", &[]);
        let limited = call(
            &mut emu,
            "kernel32.dll",
            "OpenProcess",
            &[PROCESS_QUERY_LIMITED_INFORMATION as u64, 0, pid],
        );
        assert_ne!(limited, 0);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtSetInformationProcess",
                &[limited, class, data, 4]
            ),
            STATUS_ACCESS_DENIED
        );
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtQueryInformationProcess",
                &[limited, class, data, 4, 0]
            ),
            STATUS_SUCCESS
        );
        for (buffer, length, expected) in [
            (data, 8, STATUS_INFO_LENGTH_MISMATCH),
            (data + 1, 4, STATUS_DATATYPE_MISALIGNMENT),
            (0, 4, STATUS_ACCESS_VIOLATION),
        ] {
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "NtSetInformationProcess",
                    &[current, class, buffer, length]
                ),
                expected
            );
        }
        // Opening a process can reuse the event's former slot. Close it before testing a stale key.
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[limited]),
            STATUS_SUCCESS
        );
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(query(&mut emu), 1);
        emu.set_hook_on_exception(|_, _, kind| {
            assert_eq!(kind, ExceptionType::InvalidHandle);
            false
        });
        let before = emu.fault_count;
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[0]),
            STATUS_INVALID_HANDLE
        );
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[current]),
            STATUS_SUCCESS
        );
        assert_eq!(emu.fault_count, before);
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[limited]),
            STATUS_INVALID_HANDLE
        );
        assert_eq!(emu.fault_count, before + 1);
        assert_eq!(call(&mut emu, "kernel32.dll", "CloseHandle", &[limited]), 0);
        assert_eq!(emu.fault_count, before + 2);
        emu.maps.write_dword(data, 0);
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtSetInformationProcess", &options),
            STATUS_SUCCESS
        );
        assert_eq!(query(&mut emu), 0);
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[limited]),
            STATUS_INVALID_HANDLE
        );
        assert_eq!(emu.fault_count, before + 2);
    }
}

#[test]
fn process_and_event_duplicates_keep_per_handle_attributes_across_restore() {
    use crate::windows::constants::security::SYNCHRONIZE;
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let current = if is64 {
            NT_CURRENT_PROCESS
        } else {
            NT_CURRENT_PROCESS as u32 as u64
        };
        let pid = call(&mut emu, "kernel32.dll", "GetCurrentProcessId", &[]);
        let process = call(
            &mut emu,
            "kernel32.dll",
            "OpenProcess",
            &[PROCESS_DUP_HANDLE as u64, 1, pid],
        );
        let width = if is64 { 8 } else { 4 };
        let sa = data + 32;
        emu.maps.write_dword(sa, (3 * width) as u32);
        emu.maps.write_dword(sa + 2 * width, 1);
        let name = data + 96;
        emu.maps.write_wide_string(name, "inherited event");
        let event = call(&mut emu, "kernel32.dll", "CreateEventW", &[sa, 0, 0, name]);
        let opened = call(
            &mut emu,
            "kernel32.dll",
            "OpenEventW",
            &[SYNCHRONIZE as u64, 1, name],
        );
        let flags = data + 224;
        let returned = flags + 8;
        let query = |emu: &mut Emu, handle, buffer, capacity| {
            call(
                emu,
                "ntdll.dll",
                "NtQueryObject",
                &[
                    handle,
                    OBJECT_HANDLE_FLAG_INFORMATION as u64,
                    buffer,
                    capacity,
                    returned,
                ],
            )
        };
        let assert_inherit = |emu: &mut Emu, handle, inherit| {
            emu.maps.write_dword(flags, u32::MAX);
            assert_eq!(query(emu, handle, flags, 4), STATUS_SUCCESS);
            assert_eq!(emu.maps.read_word(flags), Some(u16::from(inherit)));
            assert_eq!(emu.maps.read_word(flags + 2), Some(u16::MAX));
            assert_eq!(emu.maps.read_dword(returned), Some(2));
        };
        for (buffer, capacity) in [(0, 0), (flags, 1)] {
            emu.maps.write_dword(flags, u32::MAX);
            assert_eq!(
                query(&mut emu, event, buffer, capacity),
                STATUS_INFO_LENGTH_MISMATCH
            );
            assert_eq!(emu.maps.read_dword(flags), Some(u32::MAX));
            assert_eq!(emu.maps.read_dword(returned), Some(2));
        }
        emu.maps.write_dword(returned, u32::MAX);
        assert_eq!(query(&mut emu, event, 0, 2), STATUS_ACCESS_VIOLATION);
        assert_eq!(emu.maps.read_dword(returned), Some(u32::MAX));
        assert_inherit(&mut emu, event, true);
        assert_inherit(&mut emu, current, false);
        assert_eq!(
            emu.handle_management.waitable_id(opened),
            emu.handle_management.waitable_id(event)
        );

        let sources = [(process, PROCESS_DUP_HANDLE), (opened, SYNCHRONIZE)];
        let copies = sources.map(|(source, rights)| {
            let args = [
                current,
                source,
                current,
                data,
                rights as u64,
                0,
                DUPLICATE_SAME_ATTRIBUTES as u64,
            ];
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtDuplicateObject", &args),
                STATUS_SUCCESS
            );
            let inherited = read_pointer(&emu, data).unwrap();
            assert_ne!(source, inherited);
            let mut args = args;
            args[6] = DUPLICATE_SAME_ACCESS as u64;
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtDuplicateObject", &args),
                STATUS_SUCCESS
            );
            let plain = read_pointer(&emu, data).unwrap();
            assert_ne!(inherited, plain);
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtClose", &[source]),
                STATUS_SUCCESS
            );
            assert_eq!(query(&mut emu, source, flags, 2), STATUS_INVALID_HANDLE);
            (inherited, plain)
        });
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        for (inherited, plain) in copies {
            assert_inherit(&mut emu, inherited, true);
            assert_inherit(&mut emu, plain, false);
        }
        let [(process, _), (event, _)] = copies;
        assert_eq!(
            emu.handle_management.process_access(process),
            Some(PROCESS_DUP_HANDLE)
        );
        assert_eq!(
            emu.handle_management
                .waitable_handle(event)
                .map(|(_, access)| access),
            Some(SYNCHRONIZE)
        );
        assert_eq!(
            call(&mut emu, "ntdll.dll", "NtClose", &[process]),
            STATUS_SUCCESS
        );
        let replacement = call(
            &mut emu,
            "kernel32.dll",
            "OpenProcess",
            &[PROCESS_DUP_HANDLE as u64, 0, pid],
        );
        assert_eq!(replacement, process);
        assert_inherit(&mut emu, replacement, false);
    }
}

#[test]
fn lsa_status_translation_returns_win32_codes_and_preserves_last_error() {
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        emu.set_last_error(ERROR_ACCESS_DENIED);
        for (status, code) in [
            (STATUS_SUCCESS, ERROR_SUCCESS),
            (STATUS_NOT_IMPLEMENTED, ERROR_INVALID_FUNCTION),
            (STATUS_BUFFER_TOO_SMALL, ERROR_INSUFFICIENT_BUFFER),
            (STATUS_NO_MEMORY, ERROR_NOT_ENOUGH_MEMORY),
        ] {
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    "advapi32.dll",
                    "LsaNtStatusToWinError",
                    &[status]
                ),
                code
            );
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
        }
    }
}

#[test]
fn kernel_providers_preserve_restored_errors_across_obsolete_handle_and_language_queries() {
    for (wide, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, _) = helpers::prepare_winapi(wide);
        let call = helpers::call_winapi_export;
        let marker = 0xface;
        call(&mut emu, library, "RestoreLastError", &[marker]);
        assert_eq!(call(&mut emu, library, "SetHandleCount", &[7]), 7);
        let system = call(&mut emu, library, "GetSystemDefaultLangID", &[]);
        assert_eq!(call(&mut emu, library, "GetUserDefaultLangID", &[]), system);
        assert_eq!(
            call(&mut emu, library, "GetUserDefaultUILanguage", &[]),
            system
        );
        assert_eq!(call(&mut emu, library, "GetLastError", &[]), marker);
    }
}

#[test]
fn win64_process_parameter_strings_survive_loader_relocation() {
    let (mut emu, _) = helpers::prepare_winapi(true);
    emu.cfg.exe_name = "sample.exe".into();
    emu.cfg.arguments = "café 🐈".into();
    let params = peb64::init_arguments(&mut emu);
    let length = emu.maps.read_dword(params).unwrap() as u64;
    assert_eq!(emu.maps.read_dword(params + 4), Some(length as u32));
    let descriptors = [
        RtlUserProcessParameters64::DLL_PATH_OFFSET,
        RtlUserProcessParameters64::IMAGE_PATH_NAME_OFFSET,
        RtlUserProcessParameters64::COMMAND_LINE_OFFSET,
    ];
    for offset in descriptors {
        let text = UnicodeString64::load(params + offset, &emu.maps);
        assert!(text.buffer >= params + RtlUserProcessParameters64::size() as u64);
        assert!(text.buffer + text.maximum_length as u64 <= params + length);
        assert_eq!(
            emu.maps.read_word(text.buffer + text.length as u64),
            Some(0)
        );
    }
    let cmd = UnicodeString64::load(
        params + RtlUserProcessParameters64::COMMAND_LINE_OFFSET,
        &emu.maps,
    );
    assert!(emu.maps.read_wide_string(cmd.buffer).ends_with("café 🐈"));
}
