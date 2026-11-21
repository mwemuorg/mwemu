use csv::ReaderBuilder;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::api::abi::ArgumentFault;
use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::dispatch_api;
use crate::api::windows::common::{memory as memory_api, nt_api, trace};
use crate::api::windows::policy::{
    self, ApiDisposition, ApiMode, ApiPolicy, ApiRoute, ApiStopReason,
};
use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::serialization::Serialization;
use crate::syscall::linux::constants::errors::ENOSYS;
use crate::syscall::windows::{index_syscall_names, syscall64};
use crate::tests::helpers::{
    call_winapi32, call_winapi64, invoke_winapi_export, prepare_winapi, register_export_module,
    setup, win32_maps_folder,
};
use crate::winapi::environment_store::Encoding;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::{LIBS64_MIN, MEM_COMMIT, MEM_RESERVE};
use crate::windows::constants::process::NT_CURRENT_PROCESS;
use crate::windows::constants::syscalls::{
    WIN64_NTACCESSCHECK, WIN64_NTAREMAPPEDFILESTHESAME, WIN64_NTCLOSE, WIN64_NTQUERYIOCOMPLETION,
};
use crate::windows::constants::system::{FALSE, TRUE};
use crate::{emu_aarch64, emu32, emu64};

const CODE: u64 = 0x400000;
const DATA: u64 = 0x500000;
const MODULE: u64 = 0x70000000;
const MODULE64: u64 = LIBS64_MIN;
const MARKER: u32 = 0x12345678;
const SHADOW_SIZE: u64 = 0x88; // Shadow space and up to 13 stack arguments.

fn emulator(is64: bool, module: &str, api: &str) -> Emu {
    setup();
    let mut emu = if is64 { emu64() } else { emu32() };
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    if is64 {
        emu.init_stack64();
    } else {
        emu.init_stack32();
    }
    let base = if is64 { MODULE64 } else { MODULE };
    for (name, base) in [
        ("code".into(), CODE),
        ("data".into(), DATA),
        (format!("{module}.text"), base),
    ] {
        emu.maps
            .create_map(&name, base, 0x1000, Permission::READ_WRITE_EXECUTE)
            .unwrap();
    }
    let exports = ExportIndexData {
        export_base: 1,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Direct { rva: 0x100 })],
        named_exports: vec![NamedExport {
            name: api.into(),
            ordinal_index: 0,
        }],
    };
    register_export_module(&mut emu, &format!("{module}.dll"), base, &exports);
    emu
}

fn gateway(emu: &mut Emu, module: &str) {
    let section = format!("{module}.text");
    if emu.cfg.arch.is_64bits() {
        winapi64::gateway(MODULE64 + 0x100, &section, emu);
    } else {
        winapi32::gateway((MODULE + 0x100) as u32, &section, emu);
    }
}

#[test]
fn trace_strings_handle_unmapped_memory_and_escaped_text() {
    let (mut emu, base) = prepare_winapi(false);
    emu.maps.write_wide_string(base, "one\ntwo");
    assert_eq!(
        trace::input_string(&emu, base, Encoding::Wide),
        r#""one\ntwo""#
    );
    emu.maps.write_bytes(base, &[0x80, 0]);
    assert_eq!(trace::input_string(&emu, base, Encoding::Ansi), "\"€\"");
    assert_eq!(trace::input_string(&emu, 0, Encoding::Wide), "NULL");
    assert!(trace::input_string(&emu, u64::MAX, Encoding::Wide).contains("unreadable"));
    emu.maps.write_string(base, &"a".repeat(256));
    assert!(trace::input_string(&emu, base, Encoding::Ansi).ends_with('…'));
    assert!(!emu.api_policy.stopped());
}

#[test]
fn one_argument_handlers_can_use_the_last_stack_slot() {
    let (mut emu, _) = prepare_winapi(false);
    emu.set_api_mode(ApiMode::SimulateFail);
    let base = emu.maps.alloc(PAGE_SIZE).unwrap();
    emu.maps
        .create_map("argument_frame", base, PAGE_SIZE, Permission::READ_WRITE)
        .unwrap();
    let end = base + PAGE_SIZE;
    let sp = end - size_of::<u32>() as u64;
    assert!(emu.maps.write_dword(sp, 0));
    assert!(emu.maps.read_dword(end).is_none());
    for (library, name) in [
        ("advapi32.dll", "GetSecurityDescriptorLength"),
        ("advapi32.dll", "ImpersonateNamedPipeClient"),
        ("shell32.dll", "SHRestricted"),
        ("advapi32.dll", "LsaClearAuditLog"),
    ] {
        emu.set_sp(sp);
        invoke_winapi_export(&mut emu, library, name);
        assert!(!emu.api_policy.stopped(), "{name}");
        assert_eq!(emu.sp(), end, "{name}");
    }
}

#[test]
fn nt_fallback_preserves_guest_stack() {
    for is64 in [false, true] {
        for (name, words) in [
            ("YieldExecution", 0),
            ("SetInformationProcess", 4),
            ("SetSystemEnvironmentValueEx", 5),
            ("CreateUserProcess", 11),
            ("SetSystemTime", 2),
            ("AccessCheckByTypeResultListAndAuditAlarmByHandle", 17),
        ] {
            for prefix in ["Nt", "Zw"] {
                for skip in [false, true] {
                    let api = format!("{prefix}{name}");
                    let mut emu = emulator(is64, "ntdll", &api);
                    emu.cfg.skip_unimplemented = skip;
                    emu.set_api_mode(ApiMode::SimulateFail);
                    emu.banzai_add(&api, 99); // CSV overrides cannot change a reviewed NT recipe.
                    emu.maps.write_dword(DATA, MARKER);
                    emu.set_last_error(MARKER as u64);
                    emu.regs_mut().rsi = DATA;
                    emu.regs_mut().rax = if is64 { MODULE64 } else { MODULE } + 0x100;
                    let sp = emu.regs().rsp;
                    let mut code = vec![0x56]; // push esi/rsi
                    if is64 {
                        for offset in (0..SHADOW_SIZE).step_by(8) {
                            emu.maps
                                .write_qword(sp - 8 - SHADOW_SIZE + offset, MARKER as u64);
                        }
                        code.extend([0x48, 0x81, 0xec]); // sub rsp, shadow size
                        code.extend((SHADOW_SIZE as u32).to_le_bytes());
                    } else {
                        for _ in 0..words {
                            code.extend([0x6a, 0]);
                        } // push 0
                    }
                    code.extend([0xff, 0xd0]); // call eax/rax
                    if is64 {
                        code.extend([0x48, 0x81, 0xc4]); // caller releases its stack
                        code.extend((SHADOW_SIZE as u32).to_le_bytes());
                    }
                    code.extend([0x5e, 0x8b, 0x1e, 0x90]); // pop esi/rsi; mov ebx,[esi/rsi]; nop
                    emu.maps.write_bytes(CODE, &code);
                    emu.set_pc(CODE);
                    let end = CODE + code.len() as u64 - 1;
                    assert_eq!(emu.run(Some(end)).unwrap(), end, "{api}, x64={is64}");
                    assert_eq!(emu.regs().rsp, sp, "{api}");
                    assert_eq!(emu.regs().rsi, DATA);
                    assert_eq!(emu.regs().rbx, MARKER as u64);
                    assert_eq!(emu.regs().rax, STATUS_NOT_IMPLEMENTED);
                    if is64 {
                        for offset in (0..SHADOW_SIZE).step_by(8) {
                            assert_eq!(
                                emu.maps.read_qword(sp - 8 - SHADOW_SIZE + offset),
                                Some(MARKER as u64)
                            );
                        }
                    }
                    assert_eq!(emu.fault_count, 0);
                    assert_eq!(emu.api_policy.simulation_count, 1);
                    assert!(emu.call_stack().is_empty());
                    assert_eq!(emu.get_last_error(), MARKER as u64);
                    assert_eq!(emu.maps.read_dword(DATA), Some(MARKER));
                    assert!(!emu.process_terminated);
                }
            }
        }
    }
}

#[test]
fn nt_dispatch_preserves_handlers_and_forwarding() {
    for is64 in [false, true] {
        let modules: &[&str] = if is64 {
            &["ntdll", "kernel32", "kernelbase"]
        } else {
            &["ntdll"]
        };
        for module in modules {
            // This existing non-NTSTATUS handler is outside the fallback catalog.
            for api in ["NtGetTickCount", "ZwGetTickCount"] {
                let mut emu = emulator(is64, module, api);
                emu.cfg.skip_unimplemented = false;
                emu.tick = MARKER as usize;
                let sp = emu.regs().rsp;
                gateway(&mut emu, module);
                assert_eq!(emu.regs().rax, MARKER as u64, "{module}!{api}");
                assert_eq!(emu.regs().rsp, sp);
                assert!(!emu.process_terminated);
            }
            for api in ["NtQueryInformationProcess", "ZwQueryInformationProcess"] {
                let mut emu = emulator(is64, module, api);
                let sp = emu.regs().rsp;
                let size = if is64 { 48 } else { 24 };
                emu.maps.write_dword(DATA, MARKER);
                if is64 {
                    emu.regs_mut().rcx = u64::MAX;
                    emu.regs_mut().rdx = 0;
                    emu.regs_mut().r8 = DATA;
                    emu.regs_mut().r9 = size as u64;
                    emu.maps.write_qword(sp + 0x20, DATA + 0x100);
                } else {
                    for value in [(DATA + 0x100) as u32, size, DATA as u32, 0, u32::MAX] {
                        emu.stack_push32(value);
                    }
                }
                gateway(&mut emu, module);
                assert_eq!(emu.regs().rax, STATUS_SUCCESS);
                assert_eq!(emu.maps.read_dword(DATA), Some(STATUS_PENDING as u32));
                assert_eq!(emu.maps.read_dword(DATA + 0x100), Some(size));
                assert_eq!(emu.regs().rsp, sp);
                assert!(!emu.process_terminated);
            }
        }
    }
    for module in ["ntdll", "kernel32", "kernelbase"] {
        for (api, status) in [
            ("NtQueueApcThread", STATUS_SUCCESS),
            ("ZwQueueApcThread", STATUS_SUCCESS),
        ] {
            let mut emu = emulator(true, module, api);
            let sp = emu.regs().rsp;
            gateway(&mut emu, module);
            assert_eq!(emu.regs().rax, status, "{module}!{api}");
            assert_eq!(emu.regs().rsp, sp);
            assert!(!emu.process_terminated);
        }
    }
    // A non-native API residing in the ntdll map must still reach kernel32.
    let mut emu = emulator(true, "ntdll", "GetTickCount");
    emu.tick = MARKER as usize;
    gateway(&mut emu, "ntdll");
    assert_eq!(emu.regs().rax, MARKER as u64);
    assert!(!emu.process_terminated);
}

#[test]
fn api_fallback_respects_architecture_and_policy() {
    for is64 in [false, true] {
        for (module, api) in [
            ("ntdll", "NtUnknownCall"),
            ("ntdll", "ZwUnknownCall"),
            ("kernel32", "NtUnknownCall"),
            ("kernelbase", "ZwUnknownCall"),
            ("ntdll", "NtCurrentTeb"),
            ("ntdll", "ZwCurrentTeb"),
            ("ntdll", "NtQueryPortInformationProcess"),
            ("ntdll", "ZwQueryPortInformationProcess"),
            ("ntdll", "NtUserSetChildWindowNoActivate"),
            ("user32", "ZwUserSetChildWindowNoActivate"),
            ("ntdll", "RtlUnknownCall"),
            ("ntdll", "NtGetCurrentProcessorNumber"), // Not NTSTATUS.
            ("ntdll", "ZwGetCurrentProcessorNumber"),
            ("ntdll", "NtInitializeNlsFiles"), // Conflicting signatures.
            ("ntdll", "ZwInitializeNlsFiles"),
            ("kernel32", "UnknownCall"),
            ("advapi32", "UnknownCall"),
            ("shlwapi", "UnknownCall"),
            ("urlmon", "UnknownCall"),
            ("msvcrt", "UnknownCall"),
            ("mscoree", "UnknownCall"),
            ("ntoskrnl", "UnknownCall"),
            ("libgcc_s_dw2-1", "UnknownCall"),
            ("winmm", "waveInOpen"),
            ("gdiplus", "GdiplusStartup"),
            ("msvcp60", "UnknownCall"),
            ("missing", "NtYieldExecution"),
            ("not_loaded", "UnknownCall"),
            ("dnsapi", "UnknownCall"),
            ("api-ms-win-crt-runtime-l1-1-0", "UnknownCall"),
        ] {
            for (mode, skip, known) in [
                (None, false, true),
                (None, true, true),
                (Some(ApiMode::SimulateFail), false, true),
                (Some(ApiMode::SimulateFail), true, false),
                (Some(ApiMode::SimulateFail), true, true),
                (Some(ApiMode::SimulateSuccess), false, true),
                (Some(ApiMode::SimulateSuccess), true, false),
                (Some(ApiMode::SimulateSuccess), true, true),
            ] {
                let mut emu = emulator(is64, module, api);
                emu.cfg.skip_unimplemented = skip;
                if let Some(mode) = mode {
                    emu.set_api_mode(mode);
                }
                if known {
                    emu.banzai_add(api, 3);
                }
                let sp = emu.regs().rsp;
                emu.regs_mut().rax = MARKER as u64;
                if is64 {
                    emu.maps.write_qword(sp + 0x20, MARKER as u64);
                } else {
                    for value in [1, 2, 3] {
                        emu.stack_push32(value);
                    }
                }
                gateway(&mut emu, module);
                let native = api.starts_with("Nt") || api.starts_with("Zw");
                // The one catalog entry in this matrix takes no arguments, even
                // when its requested DLL has no handler and CSV says three words.
                let cleanup = if api == "NtYieldExecution" {
                    Some(0)
                } else if !native && skip && known {
                    Some(3)
                } else {
                    None
                };
                let simulated = emu.api_mode() != ApiMode::Strict && (is64 || cleanup.is_some());
                assert_eq!(
                    emu.api_policy.stopped(),
                    !simulated,
                    "{module}!{api}, x64={is64}"
                );
                if !simulated {
                    let reason = if emu.api_mode() == ApiMode::Strict {
                        ApiStopReason::StrictPolicy
                    } else if native || skip {
                        ApiStopReason::UnknownCleanup
                    } else {
                        ApiStopReason::BanzaiDisabled
                    };
                    assert_eq!(
                        emu.api_policy.last_outcome.as_ref().unwrap().disposition,
                        ApiDisposition::Stopped(reason)
                    );
                }
                assert!(!emu.process_terminated);
                let expected_sp = if is64 {
                    sp
                } else {
                    let cleaned = if simulated { cleanup.unwrap() } else { 0 };
                    sp - (3 - cleaned) * size_of::<u32>() as u64
                };
                assert_eq!(emu.regs().rsp, expected_sp);
                assert_eq!(emu.regs().rax, if simulated { 0 } else { MARKER as u64 });
                assert_eq!(emu.api_policy.simulation_count, simulated as u64);
                if is64 {
                    assert_eq!(emu.maps.read_qword(sp + 0x20), Some(MARKER as u64));
                }
                assert_eq!(emu.fault_count, 0);
            }
        }
    }
}

#[test]
fn nt_catalog_accounts_for_banzai_entries() {
    let path = format!("{}banzai.csv", win32_maps_folder());
    let mut csv = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();
    let mut native_entries = 0;
    for record in csv.deserialize::<(String, u32)>() {
        let (api, words) = record.unwrap();
        if !api.starts_with("Nt") && !api.starts_with("Zw") {
            continue;
        }
        native_entries += 1;
        let name = nt_api::normalize(&api);
        match name.as_ref() {
            // Existing real handler, with a non-NTSTATUS result.
            "NtGetTickCount" => assert_eq!(words, 0),
            // Non-NTSTATUS or Win32k: explicit stop, never the NTSTATUS fallback.
            "NtCurrentTeb"
            | "NtGetCurrentProcessorNumber"
            | "NtQueryPortInformationProcess"
            | "NtUserSetChildWindowNoActivate" => assert_eq!(nt_api::signature(&api), None),
            _ => assert_eq!(
                nt_api::signature(&api).map(|signature| signature.x86_words),
                Some(words),
                "{api}"
            ),
        }
    }
    assert!(native_entries > 0);
}

#[test]
fn strict_call_preserves_entry_and_does_not_resume() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "ntdll", "ZwYieldExecution");
        // Conflicting compatibility settings must still choose strict.
        emu.cfg.skip_unimplemented = true;
        emu.cfg.stop_on_unimplemented = true;
        emu.set_api_mode(ApiMode::SimulateFail);
        emu.regs_mut().rax = if is64 { MODULE64 } else { MODULE } + 0x100;
        let original_sp = emu.sp();
        emu.maps
            .write_bytes(CODE, &[0xff, 0xd0, 0xbb, 1, 0, 0, 0, 0x90]); // call eax/rax; mov ebx,1
        emu.set_pc(CODE);
        let error = emu.run(Some(CODE + 7)).unwrap_err();
        assert!(error.to_string().contains("ZwYieldExecution"));
        assert_eq!(emu.pc(), CODE);
        assert_eq!(emu.sp(), original_sp - if is64 { 8 } else { 4 });
        assert_eq!(emu.regs().rbx, 0);
        assert_eq!(emu.call_stack().len(), 1);
        assert!(!emu.process_terminated);
        assert_eq!(emu.fault_count, 0);
        let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
        assert_eq!(
            outcome.disposition,
            ApiDisposition::Stopped(ApiStopReason::StrictPolicy)
        );
        assert_eq!(outcome.call.route, ApiRoute::Address);
        assert_eq!(outcome.call.library, "ntdll");
        assert!(emu.run(Some(CODE + 7)).is_err());
        assert!(!emu.step());
        assert_eq!(emu.pc(), CODE);
    }
}

#[test]
fn invalid_simulation_frame_does_not_become_zero_arguments() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "ntdll", "NtSetSystemEnvironmentValueEx");
        emu.set_api_mode(ApiMode::SimulateFail);
        let invalid_sp = DATA + 0x2000;
        emu.set_sp(invalid_sp);
        emu.regs_mut().rax = MARKER as u64;
        gateway(&mut emu, "ntdll");
        assert_eq!(
            emu.api_policy.last_outcome.as_ref().unwrap().disposition,
            ApiDisposition::Stopped(ApiStopReason::InvalidFrame)
        );
        assert_eq!(emu.sp(), invalid_sp);
        assert_eq!(emu.regs().rax, MARKER as u64);
        assert!(!emu.process_terminated);
    }
}

#[test]
fn unresolved_imports_use_the_same_policy() {
    for mode in [
        ApiMode::Strict,
        ApiMode::SimulateFail,
        ApiMode::SimulateSuccess,
    ] {
        for dll in [
            "missing.dll",
            "msvcrt.dll",
            "api-ms-win-crt-runtime-l1-1-0.dll",
        ] {
            let mut emu = emulator(true, "ntdll", "NtYieldExecution");
            emu.set_api_mode(mode);
            emu.regs_mut().rax = MARKER as u64;
            let sp = emu.sp();
            winapi64::gateway_by_import(&mut emu, dll, "UnknownCall", DATA);
            let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
            assert_eq!(outcome.call.library, dll);
            assert_eq!(outcome.call.name, "UnknownCall");
            assert_eq!(outcome.call.route, ApiRoute::Import);
            let simulated = mode != ApiMode::Strict;
            assert_eq!(emu.api_policy.stopped(), !simulated);
            assert_eq!(emu.sp(), sp);
            assert_eq!(emu.regs().rax, if simulated { 0 } else { MARKER as u64 });
            assert_eq!(emu.api_policy.simulation_count, simulated as u64);
        }
    }
}

#[test]
fn mapped_crt_code_keeps_its_return_frame_in_strict_mode() {
    let mut emu = emulator(true, "msvcrt", "NativeCall");
    let sp = emu.sp();
    emu.maps
        .write_bytes(MODULE64 + 0x100, &[0xb8, 42, 0, 0, 0, 0xc3]); // mov eax,42; ret
    emu.maps.write_bytes(CODE, &[0xff, 0xd0, 0x90]); // call rax; nop
    emu.regs_mut().rax = MODULE64 + 0x100;
    emu.set_pc(CODE);
    assert_eq!(emu.run(Some(CODE + 2)).unwrap(), CODE + 2);
    assert_eq!(emu.regs().rax, 42);
    assert_eq!(emu.sp(), sp);
    assert!(emu.api_policy.last_outcome.is_none());
}

#[test]
fn tls_callbacks_obey_policy_and_propagate_stops() {
    for is64 in [false, true] {
        for mode in [
            ApiMode::Strict,
            ApiMode::SimulateFail,
            ApiMode::SimulateSuccess,
        ] {
            let mut emu = emulator(is64, "ntdll", "NtYieldExecution");
            emu.set_api_mode(mode);
            let sp = emu.sp();
            emu.regs_mut().rax = if is64 { MODULE64 } else { MODULE } + 0x100;
            let ret = if is64 { vec![0xc3] } else { vec![0xc2, 12, 0] };
            let mut first = vec![0xff, 0xd0]; // call rax/eax
            first.extend(&ret);
            let mut second = vec![0xbb, 42, 0, 0, 0]; // mov ebx,42
            second.extend(ret);
            emu.maps.write_bytes(CODE, &first);
            emu.maps.write_bytes(CODE + 0x100, &second);
            emu.tls_callbacks = vec![CODE, CODE + 0x100];
            let result = emu.run_tls_callbacks(DATA);
            if mode == ApiMode::Strict {
                assert!(result.is_err());
                assert!(emu.api_policy.stopped());
                assert_eq!(emu.regs().rbx, 0);
            } else {
                result.unwrap();
                assert_eq!(emu.sp(), sp);
                assert_eq!(emu.regs().rbx, 42);
                assert!(!emu.api_policy.stopped());
            }
            assert!(!emu.cfg.skip_unimplemented);
            assert!(!emu.process_terminated);
        }
    }
}

#[test]
fn api_mode_and_stop_survive_native_snapshot() {
    let mut emu = emulator(false, "ntdll", "NtUnknownCall");
    emu.set_api_mode(ApiMode::SimulateFail);
    gateway(&mut emu, "ntdll");
    let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
    assert_eq!(restored.api_mode(), ApiMode::SimulateFail);
    assert_eq!(
        restored.api_policy.last_outcome,
        emu.api_policy.last_outcome
    );
    assert!(restored.run(None).is_err());
    assert!(!restored.process_terminated);
    // Selecting API simulation has no effect on instruction/memory Banzai mode.
    assert!(!restored.cfg.skip_unimplemented);
}

#[test]
fn api_mode_is_independent_of_banzai() {
    for (mode, skip, strict, expected) in [
        (None, false, false, ApiMode::Strict),
        (None, true, false, ApiMode::Strict),
        (
            Some(ApiMode::SimulateFail),
            true,
            false,
            ApiMode::SimulateFail,
        ),
        (None, true, true, ApiMode::Strict),
        (Some(ApiMode::Strict), true, false, ApiMode::Strict),
        (
            Some(ApiMode::SimulateFail),
            false,
            false,
            ApiMode::SimulateFail,
        ),
        (Some(ApiMode::SimulateFail), false, true, ApiMode::Strict),
        (
            Some(ApiMode::SimulateSuccess),
            false,
            false,
            ApiMode::SimulateSuccess,
        ),
        (
            Some(ApiMode::SimulateSuccess),
            true,
            false,
            ApiMode::SimulateSuccess,
        ),
        (Some(ApiMode::SimulateSuccess), false, true, ApiMode::Strict),
    ] {
        let mut emu = emu32();
        emu.cfg.skip_unimplemented = skip;
        emu.cfg.stop_on_unimplemented = strict;
        if let Some(mode) = mode {
            emu.set_api_mode(mode);
        }
        assert_eq!(emu.api_mode(), expected);
        assert_eq!(emu.cfg.skip_unimplemented, skip);
        let config = serde_json::to_string(&emu.cfg).unwrap();
        emu.cfg = serde_json::from_str(&config).unwrap();
        assert_eq!(emu.api_mode(), expected);
    }
}

#[test]
fn import_resolution_stays_in_the_requested_library() {
    for is64 in [false, true] {
        for dll in [
            "missing.dll",
            "kernel32.dll",
            "api-ms-win-core-sysinfo-l1-1-0.dll",
        ] {
            let mut emu = emulator(is64, "kernel32", "GetTickCount");
            emu.tick = MARKER as usize;
            if is64 {
                winapi64::gateway_by_import(&mut emu, dll, "GetTickCount", DATA);
            } else {
                winapi32::gateway_by_import(&mut emu, dll, "GetTickCount", DATA as u32);
            }
            if dll == "missing.dll" {
                assert!(emu.api_policy.stopped());
                assert_eq!(
                    emu.api_policy.last_outcome.as_ref().unwrap().call.library,
                    dll
                );
            } else {
                assert!(emu.api_policy.last_outcome.is_none());
                assert_eq!(emu.regs().rax, MARKER as u64);
            }
        }
    }
}

#[test]
fn invalid_return_frame_stops_before_dispatch() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "ntdll", "NtYieldExecution");
        emu.set_api_mode(ApiMode::SimulateFail);
        emu.set_sp(DATA + 0x2000);
        let sp = emu.sp();
        emu.set_pc(CODE);
        if is64 {
            emu.set_rip_with_check(MODULE64 + 0x100, false);
        } else {
            emu.set_eip(MODULE + 0x100, false);
        }
        assert_eq!(
            emu.api_policy.last_outcome.as_ref().unwrap().disposition,
            ApiDisposition::Stopped(ApiStopReason::InvalidFrame)
        );
        assert_eq!(emu.api_policy.simulation_count, 0);
        assert_eq!(emu.sp(), sp);
        assert_eq!(emu.pc(), CODE);
    }
}

#[test]
fn named_system_imports_work_without_module_exports() {
    for is64 in [false, true] {
        let (mut emu, data) = prepare_winapi(is64);
        emu.set_api_mode(ApiMode::Strict);
        let invoke = |emu: &mut Emu, dll: &str, name: &str, args: &[u64]| {
            if is64 {
                call_winapi64(
                    emu,
                    |emu| winapi64::gateway_by_import(emu, dll, name, DATA),
                    args,
                )
            } else {
                u64::from(call_winapi32(
                    emu,
                    |emu| winapi32::gateway_by_import(emu, dll, name, DATA as u32),
                    &args.iter().map(|arg| *arg as u32).collect::<Vec<_>>(),
                ))
            }
        };
        for library in ["kernel32.dll", "kernelbase.dll"] {
            emu.set_last_error(ERROR_ACCESS_DENIED);
            assert_eq!(
                invoke(&mut emu, library, "GetLastError", &[]),
                ERROR_ACCESS_DENIED
            );
            emu.maps.fill_range(data, 0xcc, 24);
            assert_eq!(invoke(&mut emu, library, "GetCPInfo", &[932, data]), 1);
            assert_eq!(emu.maps.read_dword(data), Some(2));
            assert_eq!(emu.maps.read_word(data + 6), Some(0x9f81));
            assert_eq!(emu.maps.read_word(data + 18), Some(0xcccc));
            assert_eq!(invoke(&mut emu, library, "IsValidCodePage", &[0]), 0);
            assert!(!emu.api_policy.stopped());
        }
        emu.maps.write_dword(data, u32::MAX);
        invoke(&mut emu, "ntdll.dll", "RtlZeroMemory", &[data, 2]);
        assert_eq!(emu.maps.read_dword(data), Some(0xffff0000));
        assert!(!emu.api_policy.stopped());
    }
}

#[test]
fn aarch64_policy_uses_link_register_and_x0() {
    for mode in [
        ApiMode::Strict,
        ApiMode::SimulateFail,
        ApiMode::SimulateSuccess,
    ] {
        let mut emu = emu_aarch64();
        emu.init_stack_aarch64();
        emu.set_api_mode(mode);
        emu.regs_aarch64_mut().x[30] = CODE;
        emu.regs_aarch64_mut().x[0] = MARKER as u64;
        let sp = emu.sp();
        // Import entry is already post-return and must not access x86 registers.
        winapi64::gateway_by_import(&mut emu, "ntdll.dll", "NtYieldExecution", MODULE64);
        if mode == ApiMode::Strict {
            assert!(emu.api_policy.stopped());
            assert_eq!(emu.regs_aarch64().x[0], MARKER as u64);
        } else {
            assert!(!emu.api_policy.stopped());
            assert_eq!(
                emu.regs_aarch64().x[0],
                if mode == ApiMode::SimulateFail {
                    STATUS_NOT_IMPLEMENTED
                } else {
                    STATUS_SUCCESS
                }
            );
        }
        assert_eq!(emu.sp(), sp);
        assert_eq!(emu.regs_aarch64().x[30], CODE);
    }
}

#[test]
fn unresolved_library_entry_is_an_api_stop() {
    let mut emu = emulator(true, "kernel32", "KnownExport");
    emu.set_pc(CODE);
    emu.stack_push64(CODE + 2);
    let sp = emu.sp();
    emu.set_rip_with_check(MODULE64 + 0x200, true);
    assert!(emu.api_policy.stopped());
    assert!(!emu.process_terminated);
    assert_eq!(emu.sp(), sp);
    assert_eq!(emu.pc(), CODE);
    assert_eq!(
        emu.api_policy.last_outcome.as_ref().unwrap().call.address,
        MODULE64 + 0x200
    );
}

#[test]
fn declared_nt_failure_covers_the_entire_catalog() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "ntdll", "UnusedExport");
        emu.set_api_mode(ApiMode::SimulateFail);
        let sp = emu.sp();
        for &(name, signature) in nt_api::SIGNATURES {
            for api in [name.to_string(), format!("Zw{}", &name[2..])] {
                emu.api_policy.clear_outcome();
                emu.set_sp(sp);
                emu.set_last_error(MARKER as u64);
                emu.maps.write_qword(DATA, MARKER as u64);
                let invoke = |emu: &mut Emu| policy::unsupported(emu, "ntdll", &api, MODULE);
                let result = if is64 {
                    call_winapi64(&mut emu, invoke, &vec![DATA; signature.arguments as usize])
                } else {
                    call_winapi32(
                        &mut emu,
                        invoke,
                        &vec![DATA as u32; signature.x86_words as usize],
                    ) as u64
                };
                assert_eq!(result, STATUS_NOT_IMPLEMENTED, "{api}, x64={is64}");
                assert_eq!(emu.sp(), sp, "{api}");
                assert_eq!(emu.maps.read_qword(DATA), Some(MARKER as u64), "{api}");
                assert_eq!(emu.get_last_error(), MARKER as u64);
                let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
                assert_eq!(outcome.disposition, ApiDisposition::SimulatedFailure);
                assert_eq!(outcome.last_error, None);
            }
        }
        assert_eq!(
            emu.api_policy.simulation_count,
            nt_api::SIGNATURES.len() as u64 * 2
        );
    }
}

#[test]
fn native_frames_use_parameter_counts_instead_of_x86_dwords() {
    for (api, words, arguments) in [
        ("NtWow64ReadVirtualMemory64", 7, 5),
        ("NtWow64WriteVirtualMemory64", 7, 5),
        ("NtWow64QueryVirtualMemory64", 8, 6),
    ] {
        let signature = nt_api::signature(api).unwrap();
        assert_eq!(signature.x86_words, words);
        assert_eq!(signature.arguments, arguments);
        let mut emu = emulator(true, "ntdll", api);
        emu.set_api_mode(ApiMode::SimulateFail);
        // Only the declared stack arguments fit; reading the x86 slot count fails.
        let sp = DATA + 0x1000 - arguments as u64 * size_of::<u64>() as u64;
        emu.set_sp(sp);
        for index in 4..arguments {
            emu.maps
                .write_qword(sp + index as u64 * size_of::<u64>() as u64, MARKER as u64);
        }
        gateway(&mut emu, "ntdll");
        assert_eq!(emu.regs().rax, STATUS_NOT_IMPLEMENTED, "{api}");
        assert_eq!(emu.sp(), sp);
        assert!(!emu.api_policy.stopped());
    }
}

#[test]
fn success_nt_fallback_uses_catalog_and_preserves_outputs() {
    for is64 in [false, true] {
        for api in [
            "NtCreateUserProcess",
            "ZwCreateUserProcess",
            "NtSetInformationProcess",
        ] {
            let mut emu = emulator(is64, "ntdll", api);
            emu.set_api_mode(ApiMode::SimulateSuccess);
            emu.banzai_add(api, 99);
            emu.set_last_error(MARKER as u64);
            emu.maps.write_qword(DATA, MARKER as u64);
            let sp = emu.sp();
            let signature = nt_api::signature(api).unwrap();
            let result = if is64 {
                call_winapi64(
                    &mut emu,
                    |emu| gateway(emu, "ntdll"),
                    &vec![DATA; signature.arguments as usize],
                )
            } else {
                call_winapi32(
                    &mut emu,
                    |emu| gateway(emu, "ntdll"),
                    &vec![DATA as u32; signature.x86_words as usize],
                ) as u64
            };
            let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
            assert_eq!(outcome.disposition, ApiDisposition::SimulatedSuccess);
            assert_eq!(outcome.result, Some(STATUS_SUCCESS));
            assert_eq!(outcome.last_error, None);
            assert_eq!(emu.sp(), sp);
            assert_eq!(result, STATUS_SUCCESS);
            assert_eq!(emu.maps.read_qword(DATA), Some(MARKER as u64));
            assert_eq!(emu.get_last_error(), MARKER as u64);
            assert_eq!(emu.api_policy.simulation_count, 1);
        }
    }
}

#[test]
fn data_returning_handlers_keep_priority_in_all_modes() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "kernel32", "IsDebuggerPresent");
        let mut baseline = None;
        for mode in [
            ApiMode::Strict,
            ApiMode::SimulateFail,
            ApiMode::SimulateSuccess,
        ] {
            emu.set_api_mode(mode);
            emu.set_last_error(MARKER as u64);
            gateway(&mut emu, "kernel32");
            assert_eq!(emu.regs().rax, FALSE);
            let result = (emu.regs().rax, emu.get_last_error());
            if let Some(expected) = baseline {
                assert_eq!(result, expected);
            } else {
                baseline = Some(result);
            }
            assert!(emu.api_policy.last_outcome.is_none());
            assert_eq!(emu.api_policy.simulation_count, 0);
        }
    }
}

#[test]
fn all_simulation_outcomes_survive_snapshots() {
    for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
        for (library, api) in [
            ("user32", "GetScrollInfo"),
            ("unsupported_fixture", "SHGetFolderPathW"),
        ] {
            let mut emu = emulator(true, library, api);
            emu.set_api_mode(mode);
            emu.maps.write_qword(DATA, MARKER as u64);
            call_winapi64(
                &mut emu,
                |emu| gateway(emu, library),
                &[DATA, DATA + 0x100, DATA],
            );
            if library == "unsupported_fixture" {
                assert_eq!(
                    emu.api_policy.last_outcome.as_ref().unwrap().disposition,
                    ApiDisposition::SimulatedGeneric
                );
            }
            let restored = Serialization::deserialize(&Serialization::serialize(&emu));
            assert_eq!(restored.api_mode(), mode);
            assert_eq!(
                restored.api_policy.last_outcome,
                emu.api_policy.last_outcome
            );
            assert_eq!(restored.api_policy.simulation_count, 1);
            assert_eq!(restored.get_last_error(), emu.get_last_error());
            let json = serde_json::to_string(&emu.api_policy).unwrap();
            let restored_json: ApiPolicy = serde_json::from_str(&json).unwrap();
            assert_eq!(restored_json.mode, Some(mode));
            assert_eq!(restored_json.last_outcome, emu.api_policy.last_outcome);
        }
    }
}

#[test]
fn generic_fallback_returns_to_guest_without_output_writes() {
    const PATH_LOW_DWORD: u32 = 0x5556_5758;
    const PATH_MARKER: u64 = 0x5152_5354_0000_0000 | PATH_LOW_DWORD as u64;
    const ARGUMENTS: [u32; 5] = [0, 0, 0, 0, DATA as u32];
    for (is64, banzai) in [(false, true), (true, false), (true, true)] {
        for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
            let mut emu = emulator(is64, "unsupported_fixture", "SHGetFolderPathW");
            emu.set_api_mode(mode);
            emu.cfg.skip_unimplemented = banzai;
            if !is64 {
                emu.banzai_add("SHGetFolderPathW", ARGUMENTS.len() as i32);
            }
            emu.maps.write_qword(DATA, PATH_MARKER);
            emu.set_last_error(MARKER as u64);
            emu.regs_mut().rax = if is64 { MODULE64 } else { MODULE } + 0x100;
            emu.regs_mut().rsi = DATA;
            let sp = emu.sp();
            let mut code = vec![0x56]; // push esi/rsi as a stack canary
            if is64 {
                for offset in (0..SHADOW_SIZE).step_by(8) {
                    emu.maps
                        .write_qword(sp - 8 - SHADOW_SIZE + offset, MARKER as u64);
                }
                emu.maps
                    .write_qword(sp - 8 - SHADOW_SIZE + 4 * size_of::<u64>() as u64, DATA);
                code.extend([0x48, 0x81, 0xec]); // sub rsp, shadow size
                code.extend((SHADOW_SIZE as u32).to_le_bytes());
            } else {
                for argument in ARGUMENTS.into_iter().rev() {
                    code.push(0x68); // push the DWORD argument
                    code.extend(argument.to_le_bytes());
                }
            }
            code.extend([0xff, 0xd0]); // call eax/rax
            if is64 {
                code.extend([0x48, 0x81, 0xc4]); // caller releases its stack
                code.extend((SHADOW_SIZE as u32).to_le_bytes());
            }
            code.extend([0x5e, 0x8b, 0x1e, 0x90]); // pop esi/rsi; mov ebx,[esi/rsi]; nop
            emu.maps.write_bytes(CODE, &code);
            emu.set_pc(CODE);
            let end = CODE + code.len() as u64 - 1;
            assert_eq!(emu.run(Some(end)).unwrap(), end);
            assert_eq!(emu.sp(), sp);
            assert_eq!(emu.regs().rax, 0);
            assert_eq!(emu.regs().rsi, DATA);
            assert_eq!(emu.regs().rbx, PATH_LOW_DWORD as u64);
            assert_eq!(emu.maps.read_qword(DATA), Some(PATH_MARKER));
            assert_eq!(emu.get_last_error(), MARKER as u64);
            assert_eq!(emu.api_policy.simulation_count, 1);
            assert!(emu.call_stack().is_empty());
            assert_eq!(emu.cfg.skip_unimplemented, banzai);
            assert_eq!(emu.api_mode(), mode);
            let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
            assert_eq!(outcome.call.library, "unsupported_fixture");
            assert_eq!(outcome.call.name, "SHGetFolderPathW");
            assert_eq!(outcome.disposition, ApiDisposition::SimulatedGeneric);
        }
    }
}

#[test]
fn simulation_loads_banzai_defaults_without_replacing_overrides() {
    for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
        let mut emu = emulator(false, "unsupported_fixture", "SHGetFolderPathW");
        emu.cfg.maps_folder = win32_maps_folder();
        // Select the mode after the emulator and its call frame already exist.
        emu.set_api_mode(mode);
        emu.cfg.skip_unimplemented = true;
        emu.banzai_add("MessageBoxW", 2);
        let sp = emu.sp();
        let result = call_winapi32(&mut emu, |emu| gateway(emu, "unsupported_fixture"), &[0; 5]);
        assert_eq!(result, 0);
        assert_eq!(emu.api_policy.simulation_count, 1);
        assert_eq!(emu.sp(), sp);
        assert!(!emu.api_policy.stopped());
        assert_eq!(emu.banzai.get_params("SHGetFolderPathW"), 5);
        assert_eq!(emu.banzai.get_params("MessageBoxW"), 2);
        assert!(emu.cfg.skip_unimplemented);
    }
}

#[test]
fn simulation_rejects_invalid_x86_cleanup_frames() {
    for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
        for (words, sp) in [(2, DATA + 0x1000 - 4), (2, u32::MAX as u64 - 3)] {
            let mut emu = emulator(false, "unsupported_fixture", "SHGetFolderPathW");
            emu.set_api_mode(mode);
            emu.cfg.skip_unimplemented = true;
            emu.banzai_add("SHGetFolderPathW", words);
            emu.set_sp(sp);
            emu.regs_mut().rax = MARKER as u64;
            emu.set_last_error(MARKER as u64);
            gateway(&mut emu, "unsupported_fixture");
            assert_eq!(
                emu.api_policy.last_outcome.as_ref().unwrap().disposition,
                ApiDisposition::Stopped(ApiStopReason::InvalidFrame)
            );
            assert_eq!(emu.sp(), sp);
            assert_eq!(emu.regs().rax, MARKER as u64);
            assert_eq!(emu.get_last_error(), MARKER as u64);
            assert_eq!(emu.api_policy.simulation_count, 0);
        }
    }
}

#[test]
fn simulation_preserves_rejected_x86_cleanup_overrides() {
    for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
        let mut emu = emulator(false, "unsupported_fixture", "SHGetFolderPathW");
        emu.cfg.maps_folder = win32_maps_folder();
        emu.set_api_mode(mode);
        emu.cfg.skip_unimplemented = true;
        emu.banzai_add("SHGetFolderPathW", -1);
        let sp = emu.sp();
        gateway(&mut emu, "unsupported_fixture");
        assert_eq!(
            emu.api_policy.last_outcome.as_ref().unwrap().disposition,
            ApiDisposition::Stopped(ApiStopReason::UnknownCleanup)
        );
        assert_eq!(emu.banzai.get_params("SHGetFolderPathW"), -1);
        assert_eq!(emu.sp(), sp);
        assert_eq!(emu.api_policy.simulation_count, 0);
    }
}

#[test]
fn checked_arguments_stop_before_winapi_effects() {
    for is64 in [false, true] {
        for mode in [
            ApiMode::Strict,
            ApiMode::SimulateFail,
            ApiMode::SimulateSuccess,
        ] {
            let mut emu = emulator(is64, "kernel32", "VirtualAllocEx");
            emu.set_api_mode(mode);
            let stack = DATA + 0x2000;
            let size = if is64 {
                4 * size_of::<u64>()
            } else {
                4 * size_of::<u32>()
            };
            emu.maps
                .create_map(
                    "truncated_frame",
                    stack,
                    size as u64,
                    Permission::READ_WRITE,
                )
                .unwrap();
            emu.set_sp(stack);
            if is64 {
                emu.regs_mut().rcx = NT_CURRENT_PROCESS;
                emu.regs_mut().rdx = 0;
                emu.regs_mut().r8 = PAGE_SIZE;
                emu.regs_mut().r9 = (MEM_RESERVE | MEM_COMMIT) as u64;
            } else {
                for (index, value) in [
                    NT_CURRENT_PROCESS as u32,
                    0,
                    PAGE_SIZE as u32,
                    MEM_RESERVE | MEM_COMMIT,
                ]
                .into_iter()
                .enumerate()
                {
                    assert!(
                        emu.maps
                            .write_dword(stack + (index * size_of::<u32>()) as u64, value)
                    );
                }
            }
            emu.regs_mut().rax = MARKER as u64;
            emu.set_last_error(MARKER as u64);
            gateway(&mut emu, "kernel32");
            assert!(emu.api_policy.stopped(), "x64={is64}, mode={mode:?}");
            assert_eq!(emu.sp(), stack);
            assert_eq!(emu.regs().rax, MARKER as u64);
            assert_eq!(emu.get_last_error(), MARKER as u64);
        }
    }
}

#[test]
fn variadic_trace_exports_leave_x86_arguments_to_the_caller() {
    let words = [0x10, 0x20, 0, 0, 4, 0];
    for library in ["advapi32.dll", "kernelbase.dll"] {
        let (mut emu, _) = prepare_winapi(false);
        emu.set_api_mode(ApiMode::SimulateFail);
        let top = emu.sp();
        let result = call_winapi32(
            &mut emu,
            |emu| invoke_winapi_export(emu, library, "TraceMessage"),
            &words,
        );
        assert!(!emu.api_policy.stopped(), "{library}");
        assert_eq!(result, ERROR_CALL_NOT_IMPLEMENTED as u32, "{library}");
        assert_eq!(emu.sp(), top - size_of_val(&words) as u64, "{library}");
    }
}

#[test]
fn checked_arguments_stop_before_cdecl_copy() {
    let mut emu = emulator(false, "msvcrt", "memcpy");
    let stack = DATA + 0x2000;
    emu.maps
        .create_map(
            "truncated_frame",
            stack,
            2 * size_of::<u32>() as u64,
            Permission::READ_WRITE,
        )
        .unwrap();
    emu.set_sp(stack);
    assert!(emu.maps.write_dword(stack, DATA as u32));
    assert!(
        emu.maps
            .write_dword(stack + size_of::<u32>() as u64, (DATA + 16) as u32)
    );
    assert!(emu.maps.write_dword(DATA, MARKER));
    emu.regs_mut().rax = MARKER as u64;
    gateway(&mut emu, "msvcrt");
    assert!(emu.api_policy.stopped());
    assert_eq!(emu.regs().rax, MARKER as u64);
    assert_eq!(emu.sp(), stack);
    assert_eq!(emu.maps.read_dword(DATA), Some(MARKER));
}

#[test]
fn checked_arguments_stop_the_run_loop_at_the_api_entry() {
    let mut emu = emulator(true, "kernel32", "VirtualAllocEx");
    let stack = DATA + 0x2000;
    let frame_size = 5 * size_of::<u64>() as u64; // Return address and shadow space only.
    emu.maps
        .create_map(
            "truncated_call_frame",
            stack,
            frame_size,
            Permission::READ_WRITE,
        )
        .unwrap();
    emu.set_sp(stack + size_of::<u64>() as u64);
    emu.regs_mut().rax = MODULE64 + 0x100;
    emu.regs_mut().rcx = NT_CURRENT_PROCESS;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0x1000;
    emu.regs_mut().r9 = (MEM_RESERVE | MEM_COMMIT) as u64;
    emu.regs_mut().rbx = MARKER as u64;
    // call rax; xor ebx, ebx; nop. The caller must not continue after a bad frame.
    assert!(emu.maps.write_bytes(CODE, &[0xff, 0xd0, 0x31, 0xdb, 0x90]));
    emu.set_pc(CODE);
    assert!(emu.run(Some(CODE + 4)).is_err());
    assert_eq!(emu.regs().rbx, MARKER as u64);
    assert_eq!(emu.regs().rax, MODULE64 + 0x100);
    let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
    assert_eq!(emu.sp(), outcome.call.stack_pointer);
    assert_eq!(emu.pc(), outcome.call.pc);
    let ApiDisposition::Stopped(ApiStopReason::InvalidArgument(error)) = outcome.disposition else {
        panic!("unexpected outcome: {outcome:?}");
    };
    assert_eq!(error.index, 4);
    assert_eq!(error.fault, ArgumentFault::Unreadable);
}

#[test]
fn checked_syscall_arguments_record_the_syscall_frame() {
    let mut emu = emulator(true, "ntdll", "NtAllocateVirtualMemory");
    policy::begin_call(
        &mut emu,
        "kernel32",
        "outer_call",
        MODULE64,
        ApiRoute::Address,
    );
    let stack = DATA + 0x2000;
    emu.set_sp(stack);
    emu.set_pc(CODE);
    emu.regs_mut().rax = MARKER as u64;
    emu.regs_mut().rcx = NT_CURRENT_PROCESS;
    emu.regs_mut().rdx = DATA;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = DATA + 16;
    assert!(emu.maps.write_qword(DATA, 0));
    assert!(emu.maps.write_qword(DATA + 16, PAGE_SIZE));
    emu.set_last_error(MARKER as u64);
    assert!(memory_api::dispatch_nt(
        "NtAllocateVirtualMemory",
        &mut emu,
        WinSyscall
    ));
    let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
    assert_eq!(outcome.call.route, ApiRoute::Syscall);
    assert_eq!(outcome.call.pc, CODE);
    assert_eq!(outcome.call.stack_pointer, stack);
    assert_ne!(outcome.call.name, "outer_call");
    assert!(matches!(
        outcome.disposition,
        ApiDisposition::Stopped(ApiStopReason::InvalidArgument(_))
    ));
    assert_eq!(emu.sp(), stack);
    assert_eq!(emu.pc(), CODE);
    assert_eq!(emu.regs().rax, MARKER as u64);
    assert_eq!(emu.maps.read_qword(DATA), Some(0));
    assert_eq!(emu.maps.read_qword(DATA + 16), Some(PAGE_SIZE));
    assert_eq!(emu.get_last_error(), MARKER as u64);
    let restored = Serialization::deserialize(&Serialization::serialize(&emu));
    assert_eq!(
        restored.api_policy.last_outcome,
        emu.api_policy.last_outcome
    );
    assert!(restored.api_policy.stopped());
}

#[test]
fn unknown_windows_syscalls_obey_policy_at_instruction_entry() {
    let entries: &[(&str, bool, &[u8], u64)] = &[
        ("syscall", true, &[0x0f, 0x05], 0xffff),
        ("sysenter", false, &[0x0f, 0x34], 0xffff),
        ("old sysenter placeholder", false, &[0x0f, 0x34], 0x10f),
        ("int80 compatibility", false, &[0xcd, 0x80], 0xffff),
    ];
    for &(name, is64, instruction, number) in entries {
        let mut emu = emulator(is64, "ntdll", "unused_export");
        for mode in [
            ApiMode::Strict,
            ApiMode::SimulateFail,
            ApiMode::SimulateSuccess,
        ] {
            emu.api_policy.clear_outcome();
            emu.set_api_mode(mode);
            emu.set_pc(CODE);
            emu.regs_mut().rax = number;
            emu.regs_mut().rbx = MARKER as u64;
            emu.set_last_error(MARKER as u64);
            let stack = emu.sp();
            let calls = emu.call_stack().clone();
            assert!(emu.maps.write_bytes(CODE, instruction));
            // inc ebx; nop. Strict must stop before the caller continues.
            assert!(emu.maps.write_bytes(CODE + 2, &[0xff, 0xc3, 0x90]));
            let result = emu.run(Some(CODE + 4));
            assert_eq!(result.is_err(), mode == ApiMode::Strict, "{name}: {mode:?}");
            assert_eq!(emu.sp(), stack, "{name}: {mode:?}");
            assert_eq!(emu.call_stack(), &calls);
            assert_eq!(emu.get_last_error(), MARKER as u64);
            let outcome = emu
                .api_policy
                .last_outcome
                .as_ref()
                .expect("syscall outcome");
            assert_eq!(outcome.call.route, ApiRoute::Syscall);
            assert_eq!(outcome.call.pc, CODE);
            assert_eq!(outcome.call.stack_pointer, stack);
            if mode == ApiMode::Strict {
                assert_eq!(emu.pc(), CODE);
                assert_eq!(emu.regs().rax, number);
                assert_eq!(emu.regs().rbx, MARKER as u64);
            } else {
                assert_eq!(emu.regs().rax, 0);
                assert_eq!(emu.regs().rbx, (MARKER + 1) as u64);
                assert_eq!(outcome.disposition, ApiDisposition::SimulatedGeneric);
            }
        }
    }
}

const UNKNOWN_SERVICE: u64 = 0xffff;

#[test]
fn named_syscalls_use_nt_results_without_function_cleanup() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "ntdll", "unused_export");
        let instruction = if is64 { [0x0f, 0x05] } else { [0x0f, 0x34] };
        emu.syscall_name_by_real
            .insert(UNKNOWN_SERVICE, "NtQueryInformationAtom".into());
        for banzai in [false, true] {
            for (mode, expected, disposition) in [
                (
                    ApiMode::SimulateFail,
                    STATUS_NOT_IMPLEMENTED,
                    ApiDisposition::SimulatedFailure,
                ),
                (
                    ApiMode::SimulateSuccess,
                    STATUS_SUCCESS,
                    ApiDisposition::SimulatedSuccess,
                ),
            ] {
                emu.api_policy.clear_outcome();
                emu.set_api_mode(mode);
                emu.cfg.skip_unimplemented = banzai;
                emu.set_pc(CODE);
                emu.regs_mut().rax = UNKNOWN_SERVICE;
                emu.regs_mut().rdx = emu.sp();
                emu.set_last_error(MARKER as u64);
                assert!(emu.maps.write_bytes(CODE, &instruction));
                let stack = emu.sp();
                let bytes = emu.maps.read_bytes(stack, 64).to_vec();
                assert!(emu.run(Some(CODE + 2)).is_ok());
                assert_eq!(emu.regs().rax, expected);
                assert_eq!(emu.sp(), stack);
                assert_eq!(emu.maps.read_bytes(stack, 64), bytes);
                assert_eq!(emu.get_last_error(), MARKER as u64);
                let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
                assert_eq!(outcome.call.name, "NtQueryInformationAtom");
                assert_eq!(outcome.call.address, UNKNOWN_SERVICE);
                assert_eq!(outcome.disposition, disposition);
                assert!(emu.api_policy.entry.is_none());
            }
        }
    }
}

#[test]
fn syscall_hooks_keep_priority_and_outer_call_context() {
    for (is64, instruction) in [
        (true, [0x0f, 0x05]),
        (false, [0x0f, 0x34]),
        (false, [0xcd, 0x80]),
    ] {
        let mut emu = emulator(is64, "ntdll", "unused_export");
        emu.set_api_mode(ApiMode::Strict);
        emu.set_pc(CODE);
        emu.regs_mut().rax = UNKNOWN_SERVICE;
        emu.regs_mut().rbx = 0;
        policy::begin_call(&mut emu, "kernel32", "outer", MODULE64, ApiRoute::Address);
        let outer = emu.api_policy.entry.clone();
        emu.set_hook_on_syscall(|emu, number| {
            assert_eq!(number, UNKNOWN_SERVICE);
            emu.regs_mut().rax = MARKER as u64;
            emu.regs_mut().rbx += 1;
            false
        });
        assert!(emu.maps.write_bytes(CODE, &instruction));
        assert!(emu.run(Some(CODE + 2)).is_ok());
        assert_eq!(emu.regs().rax, MARKER as u64);
        assert_eq!(emu.regs().rbx, 1);
        assert_eq!(emu.api_policy.entry, outer);
        assert!(emu.api_policy.last_outcome.is_none());
    }
}

#[test]
fn syscall_frame_faults_stop_step_and_survive_snapshots() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "ntdll", "unused_export");
        let instruction = if is64 { [0x0f, 0x05] } else { [0x0f, 0x34] };
        emu.syscall_name_by_real
            .insert(UNKNOWN_SERVICE, "NtQueryInformationAtom".into());
        emu.set_api_mode(ApiMode::SimulateSuccess);
        emu.set_pc(CODE);
        emu.regs_mut().rax = UNKNOWN_SERVICE;
        // The required stack argument is outside every map.
        let stack = DATA + PAGE_SIZE;
        emu.set_sp(stack);
        emu.regs_mut().rdx = stack;
        assert!(emu.maps.write_bytes(CODE, &instruction));
        assert!(!emu.step());
        assert_eq!(emu.sp(), stack);
        assert_eq!(emu.pc(), CODE);
        assert_eq!(emu.regs().rax, UNKNOWN_SERVICE);
        let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
        assert_eq!(outcome.call.name, "NtQueryInformationAtom");
        assert_eq!(outcome.call.address, UNKNOWN_SERVICE);
        assert_eq!(
            outcome.disposition,
            ApiDisposition::Stopped(ApiStopReason::InvalidFrame)
        );
        let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(
            restored.api_policy.last_outcome,
            emu.api_policy.last_outcome
        );
        assert!(restored.run(None).is_err());
    }
}

#[test]
fn loaded_syscall_identity_prevents_static_profile_misrouting() {
    let mut emu = emulator(true, "ntdll", "unused_export");
    emu.set_api_mode(ApiMode::SimulateFail);
    emu.set_pc(CODE);
    emu.syscall_name_by_real
        .insert(WIN64_NTCLOSE, "NtQueryInformationAtom".into());
    emu.regs_mut().rax = WIN64_NTCLOSE;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_IMPLEMENTED);
    assert_eq!(
        emu.api_policy.last_outcome.as_ref().unwrap().call.name,
        "NtQueryInformationAtom"
    );

    emu.api_policy.clear_outcome();
    emu.regs_mut().rax = UNKNOWN_SERVICE;
    syscall64::gateway(&mut emu);
    let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
    assert_eq!(outcome.disposition, ApiDisposition::SimulatedGeneric);
    assert_eq!(outcome.call.address, UNKNOWN_SERVICE);
    assert_eq!(outcome.call.name, "syscall 0xffff");

    emu.api_policy.clear_outcome();
    emu.set_api_mode(ApiMode::Strict);
    emu.syscall_name_by_real
        .insert(UNKNOWN_SERVICE, "NtClose".into());
    emu.syscall_number_map
        .insert(UNKNOWN_SERVICE, WIN64_NTCLOSE);
    emu.regs_mut().rax = UNKNOWN_SERVICE;
    syscall64::gateway(&mut emu);
    assert!(emu.api_policy.last_outcome.is_none());
}

#[test]
fn strict_syscall_stop_reaches_threaded_execution() {
    let mut emu = emulator(true, "ntdll", "unused_export");
    emu.cfg.enable_threading = true;
    emu.set_pc(CODE);
    emu.regs_mut().rax = UNKNOWN_SERVICE;
    assert!(emu.maps.write_bytes(CODE, &[0x0f, 0x05, 0x90]));
    assert!(emu.run(Some(CODE + 2)).is_err());
    assert_eq!(emu.pc(), CODE);
    assert!(emu.api_policy.stopped());
}

#[test]
fn loaded_service_numbers_resolve_even_when_the_windows_build_renumbers_a_call() {
    for (name, canonical) in [
        ("NtQueryIoCompletion", Some(WIN64_NTQUERYIOCOMPLETION)),
        (
            "NtAreMappedFilesTheSame",
            Some(WIN64_NTAREMAPPEDFILESTHESAME),
        ),
        ("NtFutureService", None),
    ] {
        let mut emu = emulator(true, "ntdll", name);
        let mut stub = vec![0x4c, 0x8b, 0xd1, 0xb8];
        stub.extend_from_slice(&(UNKNOWN_SERVICE as u32).to_le_bytes());
        assert!(emu.maps.write_bytes(MODULE64 + 0x100, &stub));
        syscall64::build_syscall_translation_table(&mut emu);
        assert_eq!(
            emu.syscall_name_by_real
                .get(&UNKNOWN_SERVICE)
                .map(String::as_str),
            Some(name)
        );
        assert_eq!(
            emu.syscall_number_map.get(&UNKNOWN_SERVICE).copied(),
            canonical
        );
    }
}

#[test]
fn service_names_come_from_guest_exports() {
    for is64 in [false, true] {
        let mut emu = emulator(is64, "ntdll", "NtQueryInformationAtom");
        let address = if is64 { MODULE64 } else { MODULE } + 0x100;
        let mut stub = if is64 {
            vec![0x4c, 0x8b, 0xd1, 0xb8]
        } else {
            vec![0xb8]
        };
        stub.extend_from_slice(&(UNKNOWN_SERVICE as u32).to_le_bytes());
        assert!(emu.maps.write_bytes(address, &stub));
        if is64 {
            syscall64::build_syscall_translation_table(&mut emu);
        } else {
            emu.syscall_name_by_real = index_syscall_names(&emu);
        }
        assert_eq!(
            emu.syscall_name_by_real
                .get(&UNKNOWN_SERVICE)
                .map(String::as_str),
            Some("NtQueryInformationAtom")
        );
        assert!(emu.maps.write_byte(address, 0xc3));
        assert!(index_syscall_names(&emu).is_empty());
    }
}

#[test]
fn sysenter_simulation_returns_through_the_guest_stub() {
    let mut emu = emulator(false, "ntdll", "unused_export");
    emu.syscall_name_by_real
        .insert(UNKNOWN_SERVICE, "NtDeleteAtom".into());
    // mov eax, number; call fast_stub; ret 4; fast_stub: mov edx, esp; sysenter; ret.
    let mut code = vec![0xb8];
    code.extend_from_slice(&(UNKNOWN_SERVICE as u32).to_le_bytes());
    code.extend_from_slice(&[0xe8, 3, 0, 0, 0, 0xc2, 4, 0, 0x8b, 0xd4, 0x0f, 0x34, 0xc3]);
    assert!(emu.maps.write_bytes(CODE, &code));
    for (mode, expected) in [
        (ApiMode::SimulateFail, STATUS_NOT_IMPLEMENTED),
        (ApiMode::SimulateSuccess, STATUS_SUCCESS),
    ] {
        emu.api_policy.clear_outcome();
        emu.set_api_mode(mode);
        emu.set_pc(CODE + 0x100);
        let stack = emu.sp();
        assert_eq!(emu.call32(CODE, &[0]).unwrap(), expected as u32);
        assert_eq!(emu.sp(), stack);
        assert_eq!(emu.pc(), CODE + 0x100);
        assert_eq!(
            emu.api_policy.last_outcome.as_ref().unwrap().call.name,
            "NtDeleteAtom"
        );
    }
}

#[test]
fn linux_unknown_and_trace_only_syscalls_return_enosys() {
    const UNKNOWN_SYSCALL: u64 = 1_000_000;
    const FIRST_ARGUMENT: u64 = 0x1234;
    for (arch, instruction) in [
        (Arch::X86, &[0xcd, 0x80][..]),                 // int 0x80
        (Arch::X86_64, &[0x0f, 0x05][..]),              // syscall
        (Arch::Aarch64, &[0x01, 0x00, 0x00, 0xd4][..]), // svc #0
    ] {
        let mut emu = Emu::new(arch);
        emu.disable_ctrlc();
        emu.cfg.console_enabled = false;
        emu.running_script = true;
        emu.os = OperatingSystem::Linux;
        emu.maps
            .create_map(
                "linux.syscall",
                CODE,
                PAGE_SIZE,
                Permission::READ_WRITE_EXECUTE,
            )
            .unwrap();
        assert!(emu.maps.write_bytes(CODE, instruction));
        let numbers: &[(&str, u64)] = match arch {
            Arch::X86 => &[
                ("unknown", UNKNOWN_SYSCALL),
                ("creat", 8),
                ("execve", 11),
                ("chdir", 12),
                ("lseek", 19),
                ("mount", 21),
                ("mprotect", 125),
            ],
            Arch::X86_64 => &[
                ("unknown", UNKNOWN_SYSCALL),
                ("chdir", 80),
                ("chmod", 90),
                ("mremap", 25),
                ("dup", 32),
                ("dup2", 33),
                ("fork", 57),
                ("kill", 62),
                ("restart_syscall", 219),
            ],
            Arch::Aarch64 => &[("unknown", UNKNOWN_SYSCALL), ("mremap", 216)],
        };
        for &(name, number) in numbers {
            emu.set_pc(CODE);
            if arch.is_aarch64() {
                emu.regs_aarch64_mut().x[8] = number;
                emu.regs_aarch64_mut().x[0] = FIRST_ARGUMENT;
            } else {
                emu.regs_mut().rax = number;
                emu.regs_mut().rbx = FIRST_ARGUMENT;
                emu.regs_mut().rdi = FIRST_ARGUMENT;
            }
            assert!(emu.step());
            let result = if arch.is_aarch64() {
                emu.regs_aarch64().x[0]
            } else if arch.is_64bits() {
                emu.regs().rax
            } else {
                emu.regs().get_eax()
            };
            let expected = if arch.is_64bits() {
                ENOSYS
            } else {
                ENOSYS as u32 as u64
            };
            assert_eq!(result, expected, "{arch:?} {name}");
            assert_eq!(emu.pc(), CODE + instruction.len() as u64);
        }
    }
}

#[test]
fn linux_syscall_hooks_keep_priority_over_unknown_fallback() {
    const UNKNOWN_SYSCALL: u64 = 1_000_000;
    const HOOK_RESULT: u64 = 0x5678;
    for (arch, instruction) in [
        (Arch::X86, &[0xcd, 0x80][..]),
        (Arch::X86_64, &[0x0f, 0x05][..]),
        (Arch::Aarch64, &[0x01, 0x00, 0x00, 0xd4][..]),
    ] {
        let mut emu = Emu::new(arch);
        emu.disable_ctrlc();
        emu.cfg.console_enabled = false;
        emu.os = OperatingSystem::Linux;
        emu.maps
            .create_map(
                "linux.syscall.hook",
                CODE,
                PAGE_SIZE,
                Permission::READ_WRITE_EXECUTE,
            )
            .unwrap();
        assert!(emu.maps.write_bytes(CODE, instruction));
        emu.set_pc(CODE);
        if arch.is_aarch64() {
            emu.regs_aarch64_mut().x[8] = UNKNOWN_SYSCALL;
        } else {
            emu.regs_mut().rax = UNKNOWN_SYSCALL;
        }
        emu.hooks.on_syscall(move |emu, number| {
            assert_eq!(number, UNKNOWN_SYSCALL);
            if arch.is_aarch64() {
                emu.regs_aarch64_mut().x[0] = HOOK_RESULT;
            } else {
                emu.regs_mut().rax = HOOK_RESULT;
            }
            false
        });
        assert!(emu.step());
        let result = if arch.is_aarch64() {
            emu.regs_aarch64().x[0]
        } else {
            emu.regs().rax
        };
        assert_eq!(result, HOOK_RESULT, "{arch:?}");
        assert_eq!(emu.pc(), CODE + instruction.len() as u64);
    }
}

#[test]
fn dispatch_keeps_handler_errors_out_of_fallback() {
    fn handler(
        events: &mut Vec<&'static str>,
        outcome: Result<(), &'static str>,
    ) -> Result<(), &'static str> {
        events.push("handler");
        outcome
    }

    fn execute(name: &str, events: &mut Vec<&'static str>) -> Result<bool, &'static str> {
        let handled: Result<bool, &'static str> = dispatch_api!(name, {
            "success" | "alias" => handler(events, Ok(())),
            "error" => handler(events, Err("unreadable argument")),
        },
            handled(result) => {
                result?;
                events.push("completed");
                Ok(true)
            },
            fallback => {
                events.push("fallback");
                Ok(false)
            },
        );
        let handled = handled?;
        events.push("after dispatch");
        Ok(handled)
    }

    for name in ["success", "alias"] {
        let mut events = Vec::new();
        assert_eq!(execute(name, &mut events), Ok(true));
        assert_eq!(events, ["handler", "completed", "after dispatch"]);
    }
    let mut events = Vec::new();
    assert_eq!(execute("error", &mut events), Err("unreadable argument"));
    assert_eq!(events, ["handler"]);

    events.clear();
    assert_eq!(execute("unknown", &mut events), Ok(false));
    assert_eq!(events, ["fallback", "after dispatch"]);
}

#[test]
fn dispatch_preserves_guard_order_and_evaluates_the_selector_once() {
    for (name, expected, expected_guards) in [
        ("partial_exact", "exact", 0),
        ("partial_other", "partial", 1),
        ("unknown", "fallback", 1),
    ] {
        let mut selections = 0;
        let mut guards = 0;
        let result = dispatch_api!({
            selections += 1;
            name
        }, {
            "partial_exact" => "exact",
            symbol if {
                guards += 1;
                symbol.starts_with("partial_")
            } => "partial",
        },
            handled(value) => value,
            fallback => "fallback",
        );
        assert_eq!(result, expected, "{name}");
        assert_eq!(selections, 1, "{name}");
        assert_eq!(guards, expected_guards, "{name}");
    }
}

#[derive(Clone, Copy)]
enum UnsupportedResult {
    Win32Status,
    NtStatus,
    Hresult,
    Win32Bool,
}

const UNSUPPORTED_OPERATIONS: &[(&str, &str, usize, UnsupportedResult)] = &[
    (
        "advapi32",
        "BaseRegCreateKey",
        8,
        UnsupportedResult::Win32Status,
    ),
    (
        "advapi32",
        "LsaRemoveAccountRights",
        5,
        UnsupportedResult::NtStatus,
    ),
    (
        "advapi32",
        "AccessCheckAndAuditAlarmA",
        11,
        UnsupportedResult::Win32Bool,
    ),
    (
        "advapi32",
        "AccessCheckAndAuditAlarmW",
        11,
        UnsupportedResult::Win32Bool,
    ),
    (
        "advapi32",
        "ChangeServiceConfigW",
        11,
        UnsupportedResult::Win32Bool,
    ),
    ("shell32", "DllGetVersion", 1, UnsupportedResult::Hresult),
    (
        "crypt32",
        "CryptSIPRemoveProvider",
        1,
        UnsupportedResult::Win32Bool,
    ),
    ("ole32", "CoMarshalInterface", 6, UnsupportedResult::Hresult),
    (
        "ole32",
        "CoGetMarshalSizeMax",
        6,
        UnsupportedResult::Hresult,
    ),
    (
        "ole32",
        "OleConvertOLESTREAMToIStorageEx",
        7,
        UnsupportedResult::Hresult,
    ),
    (
        "ole32",
        "OleConvertOLESTREAMToIStorageEx2",
        10,
        UnsupportedResult::Hresult,
    ),
    (
        "crypt32",
        "CertVerifyRevocation",
        7,
        UnsupportedResult::Win32Bool,
    ),
    (
        "crypt32",
        "CryptDecodeObject",
        7,
        UnsupportedResult::Win32Bool,
    ),
    (
        "crypt32",
        "CryptDecodeObjectEx",
        8,
        UnsupportedResult::Win32Bool,
    ),
    (
        "crypt32",
        "CryptSIPCreateIndirectData",
        3,
        UnsupportedResult::Win32Bool,
    ),
    (
        "wintrust",
        "CryptSIPCreateIndirectData",
        3,
        UnsupportedResult::Win32Bool,
    ),
    ("crypt32", "CryptSIPLoad", 3, UnsupportedResult::Win32Bool),
    ("oleaut32", "VarDecMul", 3, UnsupportedResult::Hresult),
    (
        "shell32",
        "SHGetInstanceExplorer",
        1,
        UnsupportedResult::Hresult,
    ),
    ("advapi32", "AccessCheck", 8, UnsupportedResult::Win32Bool),
    ("kernelbase", "AccessCheck", 8, UnsupportedResult::Win32Bool),
    ("advapi32", "LsaOpenPolicy", 4, UnsupportedResult::NtStatus),
    ("user32", "GetPointerInfo", 2, UnsupportedResult::Win32Bool),
    (
        "crypt32",
        "CryptUnprotectMemory",
        3,
        UnsupportedResult::Win32Bool,
    ),
    (
        "wininet",
        "HttpWebSocketReceive",
        5,
        UnsupportedResult::Win32Bool,
    ),
    ("user32", "GetScrollInfo", 3, UnsupportedResult::Win32Bool),
    ("kernel32", "LockFileEx", 6, UnsupportedResult::Win32Bool),
    ("kernelbase", "LockFileEx", 6, UnsupportedResult::Win32Bool),
];

#[test]
fn unsupported_operations_use_policy_instead_of_success_handlers() {
    for is64 in [false, true] {
        for &(library, name, arguments, contract) in UNSUPPORTED_OPERATIONS {
            let mut emu = emulator(is64, library, name);
            let sp = emu.sp();
            for mode in [
                ApiMode::Strict,
                ApiMode::SimulateFail,
                ApiMode::SimulateSuccess,
            ] {
                emu.api_policy.clear_outcome();
                emu.set_api_mode(mode);
                emu.set_sp(sp);
                emu.cfg.skip_unimplemented = false;
                emu.set_last_error(MARKER as u64);
                emu.regs_mut().rax = MARKER as u64;
                emu.maps.write_qword(DATA, MARKER as u64);
                let count = emu.api_policy.simulation_count;
                if is64 {
                    call_winapi64(
                        &mut emu,
                        |emu| gateway(emu, library),
                        &vec![DATA; arguments],
                    );
                } else {
                    call_winapi32(
                        &mut emu,
                        |emu| gateway(emu, library),
                        &vec![DATA as u32; arguments],
                    );
                }
                let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
                assert_eq!(outcome.call.library, library);
                assert_eq!(outcome.call.name, name);
                let (result, error, disposition) = match (mode, contract) {
                    (ApiMode::Strict, _) => (
                        MARKER as u64,
                        MARKER as u64,
                        ApiDisposition::Stopped(ApiStopReason::StrictPolicy),
                    ),
                    (ApiMode::SimulateFail, UnsupportedResult::Win32Status) => (
                        ERROR_CALL_NOT_IMPLEMENTED,
                        MARKER as u64,
                        ApiDisposition::SimulatedFailure,
                    ),
                    (ApiMode::SimulateSuccess, UnsupportedResult::Win32Status) => (
                        ERROR_SUCCESS,
                        MARKER as u64,
                        ApiDisposition::SimulatedSuccess,
                    ),
                    (ApiMode::SimulateFail, UnsupportedResult::NtStatus) => (
                        STATUS_NOT_IMPLEMENTED,
                        MARKER as u64,
                        ApiDisposition::SimulatedFailure,
                    ),
                    (ApiMode::SimulateSuccess, UnsupportedResult::NtStatus) => (
                        STATUS_SUCCESS,
                        MARKER as u64,
                        ApiDisposition::SimulatedSuccess,
                    ),
                    (ApiMode::SimulateFail, UnsupportedResult::Hresult) => {
                        (E_NOTIMPL, MARKER as u64, ApiDisposition::SimulatedFailure)
                    }
                    (ApiMode::SimulateSuccess, UnsupportedResult::Hresult) => {
                        (S_OK, MARKER as u64, ApiDisposition::SimulatedSuccess)
                    }
                    (ApiMode::SimulateFail, UnsupportedResult::Win32Bool) => (
                        FALSE,
                        ERROR_CALL_NOT_IMPLEMENTED,
                        ApiDisposition::SimulatedFailure,
                    ),
                    (ApiMode::SimulateSuccess, UnsupportedResult::Win32Bool) => {
                        (TRUE, MARKER as u64, ApiDisposition::SimulatedSuccess)
                    }
                };
                assert_eq!(
                    outcome.disposition, disposition,
                    "{library}!{name}, x64={is64}"
                );
                assert_eq!(emu.regs().rax, result, "{library}!{name}");
                assert_eq!(emu.get_last_error(), error, "{library}!{name}");
                assert_eq!(emu.maps.read_qword(DATA), Some(MARKER as u64));
                assert_eq!(
                    emu.api_policy.simulation_count,
                    count + u64::from(mode != ApiMode::Strict)
                );
                let expected_sp = if !is64 && mode == ApiMode::Strict {
                    sp - arguments as u64 * size_of::<u32>() as u64
                } else {
                    sp
                };
                assert_eq!(emu.sp(), expected_sp, "{library}!{name}");
            }
        }
    }
}

#[test]
fn unsupported_operation_recipes_validate_the_entire_frame() {
    for is64 in [false, true] {
        for &(library, name, arguments, _) in UNSUPPORTED_OPERATIONS {
            // Register-only x64 calls have no unreadable stack argument.
            if is64 && arguments <= 4 {
                continue;
            }
            let mut emu = emulator(is64, library, name);
            let readable = if is64 {
                (arguments - 1) * 8
            } else {
                (arguments - 1) * 4
            };
            let sp = DATA + 0x1000 - readable as u64;
            for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
                emu.api_policy.clear_outcome();
                emu.set_api_mode(mode);
                emu.set_sp(sp);
                emu.set_last_error(MARKER as u64);
                emu.regs_mut().rax = MARKER as u64;
                emu.maps.write_qword(DATA, MARKER as u64);
                gateway(&mut emu, library);
                assert_eq!(
                    emu.api_policy.last_outcome.as_ref().unwrap().disposition,
                    ApiDisposition::Stopped(ApiStopReason::InvalidFrame),
                    "{library}!{name}, x64={is64}"
                );
                assert_eq!(emu.sp(), sp);
                assert_eq!(emu.regs().rax, MARKER as u64);
                assert_eq!(emu.get_last_error(), MARKER as u64);
                assert_eq!(emu.maps.read_qword(DATA), Some(MARKER as u64));
                assert_eq!(emu.api_policy.simulation_count, 0);
            }
        }
    }
}

#[test]
fn direct_access_check_leaves_grant_outputs_untouched() {
    for mode in [ApiMode::Strict, ApiMode::SimulateFail] {
        let mut emu = emulator(true, "ntdll", "unused_export");
        emu.set_api_mode(mode);
        emu.set_pc(CODE);
        emu.syscall_name_by_real
            .insert(UNKNOWN_SERVICE, "NtAccessCheck".into());
        emu.syscall_number_map
            .insert(UNKNOWN_SERVICE, WIN64_NTACCESSCHECK);
        let sp = emu.regs().rsp - SHADOW_SIZE;
        emu.regs_mut().rsp = sp;
        emu.maps.write_qword(sp + 0x38, DATA);
        emu.maps.write_qword(sp + 0x40, DATA + 4);
        emu.maps.write_dword(DATA, MARKER);
        emu.maps.write_dword(DATA + 4, MARKER);
        emu.regs_mut().rax = UNKNOWN_SERVICE;
        syscall64::gateway(&mut emu);
        let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
        assert_eq!(outcome.call.name, "NtAccessCheck");
        assert_eq!(
            emu.regs().rax,
            if mode == ApiMode::Strict {
                UNKNOWN_SERVICE
            } else {
                STATUS_NOT_IMPLEMENTED
            }
        );
        assert_eq!(emu.maps.read_dword(DATA), Some(MARKER));
        assert_eq!(emu.maps.read_dword(DATA + 4), Some(MARKER));
    }
}

#[test]
fn managed_entry_checks_the_guest_directory_and_requires_a_runtime() {
    for is64 in [false, true] {
        let (mut emu, base) = prepare_winapi(is64);

        use crate::winapi::libraries::mscoree::clr_header;
        use goblin::pe::header::{DOS_MAGIC, PE_MAGIC, PE_POINTER_OFFSET};
        use goblin::pe::optional_header::{MAGIC_32, MAGIC_64};
        emu.base = base;
        let optional = base + 0x80 + 24;
        let table = if is64 { 112 } else { 96 };
        let rva = 0x400;
        assert!(emu.maps.write_word(base, DOS_MAGIC));
        assert!(
            emu.maps
                .write_dword(base + u64::from(PE_POINTER_OFFSET), 0x80)
        );
        assert!(emu.maps.write_dword(base + 0x80, PE_MAGIC));
        assert!(emu.maps.write_word(optional - 4, (table + 16 * 8) as u16));
        assert!(
            emu.maps
                .write_word(optional, if is64 { MAGIC_64 } else { MAGIC_32 })
        );
        assert!(emu.maps.write_dword(optional + 56, 0x1000));
        assert!(emu.maps.write_dword(optional + table - 4, 16));
        assert!(emu.maps.write_dword(optional + table + 14 * 8, rva));
        assert!(emu.maps.write_dword(optional + table + 14 * 8 + 4, 72));
        assert_eq!(
            clr_header(&emu),
            Some((base + u64::from(rva), base + 0x1000))
        );
        assert!(emu.maps.write_dword(optional + table - 4, 2));
        assert_eq!(clr_header(&emu), None);
        emu.set_api_mode(ApiMode::Strict);
        invoke_winapi_export(&mut emu, "mscoree.dll", "_CorExeMain");
        assert!(emu.api_policy.stopped());
        let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
        assert_eq!(outcome.call.name, "_CorExeMain");
    }
}

#[test]
fn unsupported_exports_obey_policy_without_writing_unused_arguments() {
    use crate::tests::helpers::{call_winapi_export as call, invoke_winapi_export, prepare_winapi};
    use crate::winapi::policy::ApiMode;
    use crate::windows::constants::errors::{E_NOTIMPL, ERROR_CALL_NOT_IMPLEMENTED, S_OK};
    for is64 in [false, true] {
        for library in ["kernel32.dll", "kernelbase.dll"] {
            let (mut emu, data) = prepare_winapi(is64);
            emu.set_api_mode(ApiMode::Strict);
            invoke_winapi_export(&mut emu, library, "WerSetFlags");
            assert!(emu.api_policy.stopped());
            emu.api_policy.clear_outcome();
            emu.maps.write_dword(data, 0x12345678);
            for (mode, hresult, direct_error) in [
                (ApiMode::SimulateFail, E_NOTIMPL, ERROR_CALL_NOT_IMPLEMENTED),
                (ApiMode::SimulateSuccess, S_OK, 0),
            ] {
                emu.set_api_mode(mode);
                emu.set_last_error(127);
                for (api, words, preserves_return) in [
                    ("SetThreadpoolWait", 3, true),
                    ("SetThreadpoolWaitEx", 4, false),
                    ("SetThreadpoolTimer", 4, true),
                    ("SetThreadpoolTimerEx", 4, false),
                ] {
                    emu.regs_mut().rax = u64::from(MARKER);
                    let result = call(&mut emu, library, api, &[0; 4][..words]);
                    assert_eq!(
                        result,
                        if preserves_return {
                            u64::from(MARKER)
                        } else {
                            0
                        },
                        "{library}!{api}"
                    );
                }
                emu.regs_mut().rdx = data;
                assert_eq!(call(&mut emu, library, "WerSetFlags", &[0]), hresult);
                assert_eq!(call(&mut emu, library, "GetTapeStatus", &[0]), direct_error);
                let ring_args = if is64 {
                    vec![1, 0, 8, 8, data]
                } else {
                    vec![1, 0, 0, 8, 8, data]
                };
                assert_eq!(call(&mut emu, library, "CreateIoRing", &ring_args), hresult);
                assert_eq!(
                    call(
                        &mut emu,
                        library,
                        "CreateEnclave",
                        &[0, 0, 4096, 4096, 1, data, 0, data]
                    ),
                    0
                );
                assert_eq!(emu.maps.read_dword(data), Some(0x12345678));
                emu.set_last_error(127);
                assert_eq!(emu.get_last_error(), 127);
                assert_eq!(emu.maps.read_dword(data), Some(0x12345678));
                let success = mode == ApiMode::SimulateSuccess;
                assert_eq!(
                    call(&mut emu, library, "GetConsoleScreenBufferInfo", &[0, data]),
                    u64::from(success)
                );
                assert_eq!(
                    emu.get_last_error(),
                    if success {
                        127
                    } else {
                        ERROR_CALL_NOT_IMPLEMENTED
                    }
                );
                assert_eq!(emu.maps.read_dword(data), Some(0x12345678));
            }
        }
    }
}
