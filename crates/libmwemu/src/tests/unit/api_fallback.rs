use crate::windows::constants::errors::*;
use csv::ReaderBuilder;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::api::windows::common::nt_api;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::tests::helpers::{register_export_module, setup, win32_maps_folder};
use crate::winapi::{winapi32, winapi64};

use crate::{emu32, emu64};

const CODE: u64 = 0x400000;
const DATA: u64 = 0x500000;
const MODULE: u64 = 0x70000000;
const MODULE64: u64 = 0x170000000;
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
            ("QueryOleDirectoryFile", 11),
            ("SetHighWaitLowThread", 0),
            ("SetLowWaitHighThread", 0),
            ("Wow64GetNativeSystemInformation", 4),
            ("SetEvent", 2),
            ("SetInformationProcess", 4),
            ("FlushBuffersFileEx", 5),
            ("SetSystemEnvironmentValueEx", 5),
            ("CreateUserProcess", 11),
            ("AccessCheckByTypeResultListAndAuditAlarmByHandle", 17),
            ("Wow64ReadVirtualMemory64", 7),
            ("Wow64QueryVirtualMemory64", 8),
        ] {
            for prefix in ["Nt", "Zw"] {
                for skip in [false, true] {
                    let api = format!("{prefix}{name}");
                    let mut emu = emulator(is64, "ntdll", &api);
                    emu.cfg.skip_unimplemented = skip;
                    emu.cfg.stop_on_unimplemented = true;
                    emu.banzai_add(&api, 99); // The NT catalog takes precedence.
                    emu.maps.write_dword(DATA, MARKER);
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
            ("NtSetInformationProcess", STATUS_NOT_IMPLEMENTED),
            ("ZwSetInformationProcess", STATUS_NOT_IMPLEMENTED),
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
        for (module, api, can_skip) in [
            ("ntdll", "NtUnknownCall", false),
            ("ntdll", "ZwUnknownCall", false),
            ("kernel32", "NtUnknownCall", false),
            ("kernelbase", "ZwUnknownCall", false),
            ("ntdll", "NtCurrentTeb", false),
            ("ntdll", "ZwCurrentTeb", false),
            ("ntdll", "NtQueryPortInformationProcess", false),
            ("ntdll", "ZwQueryPortInformationProcess", false),
            ("ntdll", "NtUserSetChildWindowNoActivate", false),
            ("user32", "ZwUserSetChildWindowNoActivate", false),
            ("ntdll", "RtlUnknownCall", true),
            ("ntdll", "NtGetCurrentProcessorNumber", false), // Not NTSTATUS.
            ("ntdll", "ZwGetCurrentProcessorNumber", false),
            ("ntdll", "NtInitializeNlsFiles", false), // Conflicting signatures.
            ("ntdll", "ZwInitializeNlsFiles", false),
            ("kernel32", "UnknownCall", true),
            ("advapi32", "UnknownCall", true),
            ("dnsapi", "UnknownCall", true),
            ("api-ms-win-crt-runtime-l1-1-0", "UnknownCall", true),
        ] {
            for (skip, known) in [(false, true), (true, false), (true, true)] {
                let mut emu = emulator(is64, module, api);
                emu.cfg.skip_unimplemented = skip;
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
                let skipped = can_skip && skip && (is64 || known);
                assert_eq!(
                    emu.process_terminated, !skipped,
                    "{module}!{api}, x64={is64}"
                );
                assert_eq!(emu.regs().rsp, if is64 || skipped { sp } else { sp - 12 });
                assert_eq!(emu.regs().rax, if skipped { 0 } else { MARKER as u64 });
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
            _ => assert_eq!(nt_api::signature(&api), Some(words), "{api}"),
        }
    }
    assert!(native_entries > 0);
}
