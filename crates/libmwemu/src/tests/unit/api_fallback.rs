use csv::ReaderBuilder;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::api::abi::ArgumentFault;
use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::{memory as memory_api, nt_api};
use crate::api::windows::policy::{
    self, ApiDisposition, ApiMode, ApiPolicy, ApiRoute, ApiStopReason,
};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::serialization::Serialization;
use crate::tests::helpers::{
    call_winapi32, call_winapi64, register_export_module, setup, win32_maps_folder,
};
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::LIBS64_MIN;
use crate::windows::constants::memory::{MEM_COMMIT, MEM_RESERVE};
use crate::windows::constants::process::NT_CURRENT_PROCESS;
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
fn nt_fallback_preserves_guest_stack() {
    for is64 in [false, true] {
        for (name, words) in [
            ("YieldExecution", 0),
            ("SetInformationProcess", 4),
            ("SetSystemEnvironmentValueEx", 5),
            ("CreateUserProcess", 11),
            ("SetEvent", 2),
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
                emu.maps.write_dword(DATA, MARKER);
                if is64 {
                    emu.regs_mut().rcx = u64::MAX;
                    emu.regs_mut().rdx = 0;
                    emu.regs_mut().r8 = DATA;
                    emu.regs_mut().r9 = 48;
                    emu.maps.write_qword(sp + 0x20, DATA + 0x100);
                } else {
                    for value in [(DATA + 0x100) as u32, 48, DATA as u32, 0, u32::MAX] {
                        emu.stack_push32(value);
                    }
                }
                gateway(&mut emu, module);
                assert_eq!(emu.regs().rax, STATUS_SUCCESS);
                assert_eq!(emu.maps.read_dword(DATA), Some(0));
                assert_eq!(emu.maps.read_dword(DATA + 0x100), Some(48));
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
fn win32_simulation_updates_last_error_only_for_failure() {
    const FILE_TIME: u64 = 0x01da_1234_5678_9abc;
    const OUTPUT: u64 = DATA + 0x100;
    for api in ["FileTimeToLocalFileTime", "LocalFileTimeToFileTime"] {
        for mode in [
            ApiMode::Strict,
            ApiMode::SimulateFail,
            ApiMode::SimulateSuccess,
        ] {
            // These kernel32 exports have no x64 handler; exercise actual dispatch.
            let mut emu = emulator(true, "kernel32", api);
            emu.set_api_mode(mode);
            emu.set_last_error(MARKER as u64);
            emu.maps.write_qword(DATA, FILE_TIME);
            emu.maps.write_qword(OUTPUT - 8, MARKER as u64);
            emu.maps.write_qword(OUTPUT, MARKER as u64);
            emu.maps.write_qword(OUTPUT + 8, MARKER as u64);
            emu.regs_mut().rax = MARKER as u64;
            let sp = emu.sp();
            let result = call_winapi64(&mut emu, |emu| gateway(emu, "kernel32"), &[DATA, OUTPUT]);
            let (expected, output, error, disposition) = match mode {
                ApiMode::Strict => (
                    MARKER as u64,
                    MARKER as u64,
                    MARKER as u64,
                    ApiDisposition::Stopped(ApiStopReason::StrictPolicy),
                ),
                ApiMode::SimulateFail => (
                    FALSE,
                    MARKER as u64,
                    ERROR_CALL_NOT_IMPLEMENTED,
                    ApiDisposition::SimulatedFailure,
                ),
                ApiMode::SimulateSuccess => (
                    TRUE,
                    FILE_TIME,
                    MARKER as u64,
                    ApiDisposition::SimulatedSuccess,
                ),
            };
            assert_eq!(result, expected, "{api}: {mode:?}");
            assert_eq!(emu.sp(), sp);
            assert_eq!(emu.maps.read_qword(OUTPUT), Some(output));
            assert_eq!(emu.maps.read_qword(DATA), Some(FILE_TIME));
            assert_eq!(emu.maps.read_qword(OUTPUT - 8), Some(MARKER as u64));
            assert_eq!(emu.maps.read_qword(OUTPUT + 8), Some(MARKER as u64));
            assert_eq!(emu.get_last_error(), error);
            let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
            assert_eq!(outcome.disposition, disposition);
            assert_eq!(
                outcome.last_error,
                (mode == ApiMode::SimulateFail).then_some(ERROR_CALL_NOT_IMPLEMENTED)
            );
        }
    }
}

#[test]
fn success_output_faults_do_not_publish_success() {
    for (label, input, output) in [
        ("null input", 0, DATA + 0x100),
        ("unmapped input", DATA + 0x2000, DATA + 0x100),
        ("truncated input", DATA + 0xffc, DATA + 0x100),
        ("null output", DATA, 0),
        ("truncated output", DATA, DATA + 0xffc),
        ("overflowing output", DATA, u64::MAX - 3),
        ("same buffer", DATA, DATA),
        ("read-only output", DATA, CODE),
    ] {
        let mut emu = emulator(true, "kernel32", "FileTimeToLocalFileTime");
        emu.set_api_mode(ApiMode::SimulateSuccess);
        emu.maps.write_qword(DATA, MARKER as u64);
        emu.maps.write_qword(DATA + 0x100, MARKER as u64);
        emu.maps.write_dword(DATA + 0xffc, MARKER);
        emu.maps
            .get_mem_by_addr_mut(CODE)
            .unwrap()
            .set_permission(Permission::READ);
        emu.set_last_error(MARKER as u64);
        emu.regs_mut().rax = MARKER as u64;
        let sp = emu.sp();
        call_winapi64(&mut emu, |emu| gateway(emu, "kernel32"), &[input, output]);
        assert_eq!(
            emu.api_policy.last_outcome.as_ref().unwrap().disposition,
            ApiDisposition::Stopped(ApiStopReason::InvalidMemory),
            "{label}"
        );
        assert_eq!(emu.regs().rax, MARKER as u64);
        assert_eq!(emu.sp(), sp);
        assert_eq!(emu.get_last_error(), MARKER as u64);
        assert_eq!(emu.maps.read_qword(DATA + 0x100), Some(MARKER as u64));
        assert_eq!(emu.maps.read_dword(DATA + 0xffc), Some(MARKER));
        assert_eq!(emu.api_policy.simulation_count, 0);
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
            ("kernel32", "FileTimeToLocalFileTime"),
            ("shfolder", "SHGetFolderPathW"),
        ] {
            let mut emu = emulator(true, library, api);
            emu.set_api_mode(mode);
            emu.maps.write_qword(DATA, MARKER as u64);
            call_winapi64(&mut emu, |emu| gateway(emu, library), &[DATA, DATA + 0x100]);
            if library == "shfolder" {
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
fn shfolder_generic_fallback_returns_to_guest_without_output_writes() {
    const PATH_LOW_DWORD: u32 = 0x5556_5758;
    const PATH_MARKER: u64 = 0x5152_5354_0000_0000 | PATH_LOW_DWORD as u64;
    const ARGUMENTS: [u32; 5] = [0, 0, 0, 0, DATA as u32];
    for (is64, banzai) in [(false, true), (true, false), (true, true)] {
        for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
            let mut emu = emulator(is64, "shfolder", "SHGetFolderPathW");
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
            assert_eq!(outcome.call.library, "shfolder");
            assert_eq!(outcome.call.name, "SHGetFolderPathW");
            assert_eq!(outcome.disposition, ApiDisposition::SimulatedGeneric);
        }
    }
}

#[test]
fn simulation_loads_banzai_defaults_without_replacing_overrides() {
    for mode in [ApiMode::SimulateFail, ApiMode::SimulateSuccess] {
        let mut emu = emulator(false, "shfolder", "SHGetFolderPathW");
        emu.cfg.maps_folder = win32_maps_folder();
        // Select the mode after the emulator and its call frame already exist.
        emu.set_api_mode(mode);
        emu.cfg.skip_unimplemented = true;
        emu.banzai_add("MessageBoxW", 2);
        let sp = emu.sp();
        let result = call_winapi32(&mut emu, |emu| gateway(emu, "shfolder"), &[0; 5]);
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
            let mut emu = emulator(false, "shfolder", "SHGetFolderPathW");
            emu.set_api_mode(mode);
            emu.cfg.skip_unimplemented = true;
            emu.banzai_add("SHGetFolderPathW", words);
            emu.set_sp(sp);
            emu.regs_mut().rax = MARKER as u64;
            emu.set_last_error(MARKER as u64);
            gateway(&mut emu, "shfolder");
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
        let mut emu = emulator(false, "shfolder", "SHGetFolderPathW");
        emu.cfg.maps_folder = win32_maps_folder();
        emu.set_api_mode(mode);
        emu.cfg.skip_unimplemented = true;
        emu.banzai_add("SHGetFolderPathW", -1);
        let sp = emu.sp();
        gateway(&mut emu, "shfolder");
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
