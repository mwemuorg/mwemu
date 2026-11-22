use iced_x86::Register;
use itertools::iproduct;
use std::sync::atomic::Ordering;
use std::thread;

use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::tests::helpers::{call_winapi_export, prepare_winapi, register_export_module, setup};
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::errors::{ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER};
use crate::{emu32, emu64};

const MODULE64: u64 = 0x170000000;
const MODULE32: u64 = 0x70000000;
const SENTINEL: u64 = 0xDEAD_C0DE_CAFE_BABE;

const NONVOLATILE32: &[Register] = &[Register::EBX, Register::EBP, Register::ESI, Register::EDI];
const NONVOLATILE64: &[Register] = &[
    Register::RBX,
    Register::RBP,
    Register::RSI,
    Register::RDI,
    Register::R12,
    Register::R13,
    Register::R14,
    Register::R15,
];
const NONVOLATILE_XMM64: &[Register] = &[
    Register::XMM6,
    Register::XMM7,
    Register::XMM8,
    Register::XMM9,
    Register::XMM10,
    Register::XMM11,
    Register::XMM12,
    Register::XMM13,
    Register::XMM14,
    Register::XMM15,
];

fn check_handler_abi(is64: bool) {
    setup();
    let (mut emu, data) = prepare_winapi(is64);
    emu.running_script = true;
    emu.cfg.skip_unimplemented = false;
    emu.cfg.stop_on_unimplemented = true;
    let nonvolatile = if is64 { NONVOLATILE64 } else { NONVOLATILE32 };
    for (index, &register) in nonvolatile.iter().enumerate() {
        emu.regs_mut().set_reg(register, SENTINEL + index as u64);
    }
    if is64 {
        for (index, &register) in NONVOLATILE_XMM64.iter().enumerate() {
            let value = ((SENTINEL as u128) << u64::BITS) | (SENTINEL + index as u64) as u128;
            emu.regs_mut().set_xmm_reg(register, value);
        }
    }
    let initial_regs = *emu.regs();

    // BSTR lengths count the embedded NUL and differ in bytes and UTF-16 units.
    let bstr = data + size_of::<u32>() as u64;
    assert!(emu.maps.write_dword(data, 6));
    emu.maps.write_wide_string(bstr, "A\0B");

    // Bounded, read-only cases, not an inventory of implemented exports.
    // Every case supplies its own arguments and checks an observable result.
    let cases: &[(&str, &str, &[u64], u64)] = &[
        ("kernel32.dll", "GetACP", &[], 1252),
        ("kernel32.dll", "GetLastError", &[], ERROR_ACCESS_DENIED),
        ("user32.dll", "IsCharAlphaA", &[b'A' as u64], 1),
        ("user32.dll", "IsCharAlphaW", &[b'7' as u64], 0),
        ("user32.dll", "IsCharAlphaNumericA", &[b'7' as u64], 1),
        ("user32.dll", "IsCharAlphaNumericW", &[b'!' as u64], 0),
        ("user32.dll", "IsCharLowerA", &[b'a' as u64], 1),
        ("user32.dll", "IsCharLowerW", &[b'A' as u64], 0),
        ("user32.dll", "IsCharUpperA", &[b'A' as u64], 1),
        ("user32.dll", "IsCharUpperW", &[b'a' as u64], 0),
        ("ws2_32.dll", "htons", &[0x1234], 0x3412),
        ("ws2_32.dll", "htonl", &[0x1234_5678], 0x7856_3412),
        ("ws2_32.dll", "ntohs", &[0x1234], 0x3412),
        ("ws2_32.dll", "ntohl", &[0x1234_5678], 0x7856_3412),
        (
            "iphlpapi.dll",
            "GetIfTable",
            &[0, 0, 0],
            ERROR_INVALID_PARAMETER,
        ),
        (
            "iphlpapi.dll",
            "GetIpAddrTable",
            &[0, 0, 0],
            ERROR_INVALID_PARAMETER,
        ),
        (
            "iphlpapi.dll",
            "GetTcpTable",
            &[0, 0, 0],
            ERROR_INVALID_PARAMETER,
        ),
        (
            "iphlpapi.dll",
            "GetUdpTable",
            &[0, 0, 0],
            ERROR_INVALID_PARAMETER,
        ),
        (
            "iphlpapi.dll",
            "GetAdaptersInfo",
            &[0, 0],
            ERROR_INVALID_PARAMETER,
        ),
        (
            "iphlpapi.dll",
            "GetAdaptersAddresses",
            &[0, 0, 0, 0, 0],
            ERROR_INVALID_PARAMETER,
        ),
        (
            "iphlpapi.dll",
            "GetNetworkParams",
            &[0, 0],
            ERROR_INVALID_PARAMETER,
        ),
        ("oleaut32.dll", "SysStringLen", &[bstr], 3),
        ("oleaut32.dll", "SysStringByteLen", &[bstr], 6),
    ];

    for &(library, api, arguments, expected) in cases {
        // These calls have no persistent effects except LastError and registers.
        // Keep the module registry so successive names exercise export replacement.
        *emu.regs_mut() = initial_regs;
        emu.regs_mut().rax = SENTINEL;
        emu.set_last_error(ERROR_ACCESS_DENIED);
        emu.is_running.store(1, Ordering::Relaxed);
        eprintln!(
            "checking {library}!{api} ({}-bit)",
            if is64 { 64 } else { 32 }
        );

        let result = call_winapi_export(&mut emu, library, api, arguments);
        assert_eq!(result, expected, "{library}!{api} result");
        assert_eq!(
            emu.is_running.load(Ordering::Relaxed),
            1,
            "{library}!{api} stopped emulation"
        );
        assert!(
            !emu.process_terminated,
            "{library}!{api} terminated the guest"
        );
        for &register in nonvolatile {
            assert_eq!(
                emu.regs().get_reg(register),
                initial_regs.get_reg(register),
                "{library}!{api} changed {register:?}"
            );
        }
        if is64 {
            for &register in NONVOLATILE_XMM64 {
                assert_eq!(
                    emu.regs().get_xmm_reg(register),
                    initial_regs.get_xmm_reg(register),
                    "{library}!{api} changed {register:?}"
                );
            }
        }
    }
}

#[test]
fn winapi32_handlers_preserve_call_frame() {
    check_handler_abi(false);
}

#[test]
fn winapi64_handlers_preserve_call_frame() {
    check_handler_abi(true);
}

fn register_dll(emu: &mut Emu, dll: &str, api: &str, base: u64) {
    emu.maps
        .create_map(
            &format!("{dll}.text"),
            base,
            0x1000,
            Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    let exports = ExportIndexData {
        export_base: 1,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Direct { rva: 0x100 })],
        named_exports: vec![NamedExport {
            name: api.into(),
            ordinal_index: 0,
        }],
    };
    register_export_module(emu, &format!("{dll}.dll"), base, &exports);
}

fn make_emu64_zeroed(dll: &str, api: &str) -> Emu {
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack64();
    register_dll(&mut emu, dll, api, MODULE64);

    emu.regs_mut().rcx = 0;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    let rsp = emu.regs().rsp;
    for i in 0..16 {
        emu.maps.write_qword(rsp + 0x20 + i * 8, 0);
    }
    emu
}

const ARENA: u64 = 0x600000;

fn make_emu64_with_arena(dll: &str, api: &str) -> (Emu, u64) {
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack64();
    register_dll(&mut emu, dll, api, MODULE64);

    emu.maps
        .create_map("arena", ARENA, 0x10000, Permission::READ_WRITE)
        .unwrap();

    // rcx points into arena (valid pointer, small enough if misread as size).
    // rdx points into arena (the sentinel target).
    // r8/r9 zero (safe sizes).
    emu.regs_mut().rcx = ARENA + 0x100;
    emu.regs_mut().rdx = ARENA + 0x2000;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    let rsp = emu.regs().rsp;
    for i in 0..16 {
        emu.maps.write_qword(rsp + 0x20 + i * 8, 0);
    }
    (emu, ARENA)
}

fn make_emu32_zeroed(dll: &str, api: &str) -> Emu {
    let mut emu = emu32();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack32();
    register_dll(&mut emu, dll, api, MODULE32);

    for _ in 0..20 {
        emu.stack_push32(0);
    }
    emu
}

/// Spot-check that APIs known NOT to use rdx as an output pointer don't
/// write to it. This catches the mechanical `write_qword(rdx, 0)` pattern
/// that the 58433ebf regression introduced across ~1,400 handlers.
#[test]
fn winapi64_non_output_apis_dont_write_rdx() {
    setup();

    // Representative APIs where rdx is NOT an output pointer.
    let cases: &[(&str, &str, &str)] = &[
        ("kernel32", "kernel32.text", "GetLocaleInfoW"),
        ("kernel32", "kernel32.text", "SetLastError"),
        ("kernel32", "kernel32.text", "Sleep"),
        ("kernel32", "kernel32.text", "GetTickCount"),
        ("kernel32", "kernel32.text", "GetCurrentProcessId"),
        ("kernel32", "kernel32.text", "GetCurrentThreadId"),
        ("kernel32", "kernel32.text", "ExitProcess"),
        ("kernel32", "kernel32.text", "FreeLibrary"),
        ("kernel32", "kernel32.text", "CloseHandle"),
        ("user32", "user32.text", "ShowWindow"),
        ("user32", "user32.text", "SetWindowTextA"),
        ("user32", "user32.text", "DestroyWindow"),
        ("user32", "user32.text", "PostMessageA"),
        ("user32", "user32.text", "MessageBoxA"),
        ("gdi32", "gdi32.text", "RestoreDC"),
        ("gdi32", "gdi32.text", "SelectObject"),
        ("gdi32", "gdi32.text", "DeleteObject"),
        ("advapi32", "advapi32.text", "RegCloseKey"),
        ("ws2_32", "ws2_32.text", "closesocket"),
        ("ws2_32", "ws2_32.text", "htons"),
    ];

    for &(dll, section, api) in cases {
        let (mut emu, _arena) = make_emu64_with_arena(dll, api);

        let rdx_addr = emu.regs().rdx;
        emu.maps.write_qword(rdx_addr, SENTINEL);

        winapi64::gateway(MODULE64 + 0x100, section, &mut emu);

        assert_eq!(
            emu.maps.read_qword(rdx_addr),
            Some(SENTINEL),
            "{dll}!{api} must not write through RDX"
        );
    }
}

#[test]
fn large_gateways_fit_a_one_mebibyte_host_stack() {
    const HOST_STACK_SIZE: usize = 1024 * 1024;
    setup();
    for (is64, dll) in iproduct!(
        [false, true],
        [
            "kernel32",
            "kernelbase",
            "advapi32",
            "user32",
            "shell32",
            "ole32"
        ]
    ) {
        thread::Builder::new()
            .stack_size(HOST_STACK_SIZE)
            .spawn(move || {
                let mut emu = if is64 {
                    make_emu64_zeroed(dll, "MissingApiForStackCheck")
                } else {
                    make_emu32_zeroed(dll, "MissingApiForStackCheck")
                };
                emu.cfg.skip_unimplemented = false;
                emu.is_running.store(1, Ordering::Relaxed);
                let section = format!("{dll}.text");
                if is64 {
                    winapi64::gateway(MODULE64 + 0x100, &section, &mut emu);
                } else {
                    winapi32::gateway((MODULE32 + 0x100) as u32, &section, &mut emu);
                }
                assert_eq!(emu.is_running.load(Ordering::Relaxed), 0);
            })
            .unwrap()
            .join()
            .unwrap();
    }
}

#[test]
fn kernelbase_preserves_the_unimplemented_api_result() {
    let mut emu = make_emu64_zeroed("kernelbase", "MissingKernelbaseApi");
    emu.cfg.skip_unimplemented = false;
    assert_eq!(
        winapi64::kernelbase::gateway(MODULE64 + 0x100, &mut emu),
        "missingkernelbaseapi"
    );
}

#[test]
fn winapi_stdcall_discards_sdk_dword_slots() {
    setup();
    let (mut emu, _) = prepare_winapi(false);
    emu.set_api_mode(crate::ApiMode::SimulateFail);
    // Exercise both handlers and known fallback signatures. COORD takes one x86 slot, CY and TRACEHANDLE take two.
    let cases = [
        ("advapi32.dll", "AbortSystemShutdownA", 1),
        ("advapi32.dll", "AbortSystemShutdownW", 1),
        ("advapi32.dll", "ControlTraceW", 5),
        ("kernel32.dll", "GetEnabledXStateFeatures", 0),
        ("kernel32.dll", "GetThreadEnabledXStateFeatures", 0),
        ("kernel32.dll", "GetPhysicallyInstalledSystemMemory", 1),
        ("user32.dll", "EnumWindows", 2),
        ("user32.dll", "EnumChildWindows", 3),
        ("user32.dll", "SetWindowPos", 7),
        ("user32.dll", "GetDlgItem", 2),
        ("ole32.dll", "CoWaitForMultipleHandles", 5),
        ("oleaut32.dll", "VarI4FromCy", 3),
        ("crypt32.dll", "CertCompareCertificate", 3),
        ("crypt32.dll", "CertVerifyCertificateChainPolicy", 4),
        ("wininet.dll", "InternetSetOptionA", 4),
        ("wininet.dll", "InternetQueryOptionW", 4),
        ("ws2_32.dll", "WSAWaitForMultipleEvents", 5),
    ];
    let stack = emu.regs().rsp;
    emu.maps.write_dword(stack, SENTINEL as u32);
    for (library, api, words) in cases {
        call_winapi_export(&mut emu, library, api, &vec![0; words]);
        assert_eq!(emu.maps.read_dword(stack), Some(SENTINEL as u32), "{api}");
    }
}

#[test]
fn winapi_output_writes_respect_argument_positions_and_types() {
    setup();
    let (mut emu, data) = prepare_winapi(true);
    emu.maps.write_qword(data, SENTINEL);
    emu.maps
        .write_qword(data + size_of::<u64>() as u64, SENTINEL);
    call_winapi_export(
        &mut emu,
        "advapi32.dll",
        "GetEventLogInformation",
        &[0, 0, 0, 0, data, data + 8],
    );
    assert_eq!(emu.maps.read_dword(data), Some(size_of::<u32>() as u32));
    assert_eq!(
        emu.maps.read_dword(data + 4),
        Some((SENTINEL >> u32::BITS) as u32)
    );
    assert_eq!(emu.maps.read_qword(data + 8), Some(SENTINEL));
    // The first stack argument is the creation disposition, an input DWORD.
    let path = data + 128;
    emu.maps
        .write_wide_string(path, r"C:\property-invalid-disposition");
    call_winapi_export(
        &mut emu,
        "kernelbase.dll",
        "CreateFileW",
        &[path, 0, 0, 0, data + 8, 0, 0],
    );
    assert_eq!(emu.maps.read_qword(data + 8), Some(SENTINEL));
    // These register arguments are flags, sizes or application data, not outputs.
    for (library, api, arguments) in [
        (
            "kernel32.dll",
            "InitializeCriticalSectionEx",
            vec![0, 0, data + 8],
        ),
        ("advapi32.dll", "EventProviderEnabled", vec![0, 0, data + 8]),
        ("user32.dll", "EnumChildWindows", vec![0, 0, data + 8]),
        ("kernelbase.dll", "WaitOnAddress", vec![0, 0, 0, data + 8]),
    ] {
        call_winapi_export(&mut emu, library, api, &arguments);
        assert_eq!(emu.maps.read_qword(data + 8), Some(SENTINEL), "{api}");
    }
    emu.set_api_mode(crate::ApiMode::SimulateFail);
    for (library, api, arguments) in [
        ("advapi32.dll", "EventAccessQuery", vec![0, 0, data]),
        ("ws2_32.dll", "WSCEnumProtocols32", vec![0, 0, data, 0]),
    ] {
        emu.maps.write_qword(data, SENTINEL);
        call_winapi_export(&mut emu, library, api, &arguments);
        assert_eq!(emu.maps.read_qword(data), Some(SENTINEL), "{api}");
        assert_eq!(emu.maps.read_qword(data + 8), Some(SENTINEL), "{api}");
    }
}

#[test]
fn winapi_qword_results_replace_both_halves_on_x86() {
    setup();
    let mut emu = emu32();
    for (library, api, arguments) in [
        ("kernel32.dll", "GetEnabledXStateFeatures", vec![]),
        ("kernel32.dll", "GetThreadEnabledXStateFeatures", vec![]),
        ("wininet.dll", "CreateUrlCacheGroup", vec![0, 0]),
    ] {
        emu.regs_mut().rdx = u32::MAX as u64;
        call_winapi_export(&mut emu, library, api, &arguments);
        assert_eq!(emu.regs().rdx, 0, "{api} must replace EDX");
    }
    const HIGH_MASK: u64 = 0x1234_5678_0000_0000;
    const VER_MAJORVERSION: u64 = 2;
    const VER_GREATER_EQUAL: u64 = 3;
    for library in ["kernel32.dll", "ntdll.dll"] {
        call_winapi_export(
            &mut emu,
            library,
            "VerSetConditionMask",
            &[
                HIGH_MASK as u32 as u64,
                HIGH_MASK >> u32::BITS,
                VER_MAJORVERSION,
                VER_GREATER_EQUAL,
            ],
        );
        let mask = (emu.regs().get_edx() << u32::BITS) | emu.regs().get_eax();
        assert_eq!(mask, HIGH_MASK | (VER_GREATER_EQUAL << 3));
    }
}
