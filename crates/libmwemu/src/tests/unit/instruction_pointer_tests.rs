// Tests for instruction pointer operations
// Note: Most set_rip/set_eip functionality requires complex setup
// These tests verify basic behavior

#[test]
fn test_set_rip_nonmapped_linux() {
    let mut emu = crate::emu64();
    emu.os = crate::arch::OperatingSystem::Linux;

    let result = emu.set_rip_with_check(0xdeadbeef, false);
    assert!(!result);
}

#[test]
fn test_set_eip_nonmapped_linux() {
    let mut emu = crate::emu32();
    emu.os = crate::arch::OperatingSystem::Linux;

    let result = emu.set_eip(0xdeadbeef, false);
    assert!(!result);
}

#[test]
fn test_force_reload_flag_exists() {
    let mut emu = crate::emu64();

    emu.force_reload = false;
    assert!(!emu.force_reload);

    emu.force_reload = true;
    assert!(emu.force_reload);
}

#[test]
fn test_os_default_is_windows() {
    let emu = crate::emu64();
    assert!(emu.os.is_windows());
}

#[test]
fn test_skip_apicall_flag() {
    let mut emu = crate::emu64();

    emu.skip_apicall = false;
    assert!(!emu.skip_apicall);

    emu.skip_apicall = true;
    assert!(emu.skip_apicall);
}

// ---------------------------------------------------------------------------
// Native x64 msvcrt execution regressions.
//
// These tests verify that legacy `msvcrt.dll` calls land in the mapped
// `msvcrt.text` / `msvcrtfothk` machine code instead of running any Rust-side
// emulation. They cover the direct mapped call path and the unresolved IAT
// thunk path inside `Emu::set_rip_with_check`, and the manual
// `Emu::handle_winapi` path. No sample bundle or network is required.

use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

const MODULE_BASE: u64 = 0x7FF0_0000_0000;
const NATIVE_MAP_BASE: u64 = MODULE_BASE + 0x1_000;
const STACK_TOP: u64 = 0x8000_0000_0800;
const CALLER_IP: u64 = 0x0001_0000;
const CALLER_NEXT: u64 = CALLER_IP + 2;

/// One fully prepared x64 emulator wired for native msvcrt execution.
fn build_msvcrt_native_emu(map_name: &str, export_name: &str) -> crate::emu::Emu {
    use crate::maps::mem64::Permission;
    use crate::tests::helpers;

    let mut emu = crate::emu64();
    emu.os = crate::arch::OperatingSystem::Windows;
    emu.cfg.verbose = 0;
    emu.cfg.nocolors = true;

    emu.maps
        .create_map(
            "msvcrt_native_caller",
            CALLER_IP,
            0x400,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("create caller map");
    emu.maps.write_bytes(CALLER_IP, &[0xFF, 0xD0, 0x90, 0x90]);

    emu.maps
        .create_map(map_name, NATIVE_MAP_BASE, 0x1_000, Permission::READ_EXECUTE)
        .expect("create native map");
    emu.maps.write_byte(NATIVE_MAP_BASE, 0xC3);

    emu.maps
        .create_map(
            "msvcrt_native_stack",
            0x8000_0000_0000u64,
            0x1_000,
            Permission::READ_WRITE,
        )
        .expect("create stack map");
    emu.regs_mut().rsp = STACK_TOP;

    let parsed = ExportIndexData {
        export_base: 1,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Direct { rva: 0x1_000 })],
        named_exports: vec![NamedExport {
            name: export_name.to_string(),
            ordinal_index: 0,
        }],
    };
    helpers::register_export_module(&mut emu, "msvcrt.dll", MODULE_BASE, &parsed);
    emu
}

#[test]
fn msvcrt_native_executes_text_section_bytes() {
    let mut emu = build_msvcrt_native_emu("msvcrt.text", "_initterm");

    use std::cell::RefCell;
    use std::rc::Rc;
    let calls: Rc<RefCell<Vec<u64>>> = Rc::new(RefCell::new(Vec::new()));
    let calls_for_hook = Rc::clone(&calls);
    emu.hooks.on_winapi_call(move |_emu, _rip, called_addr| {
        calls_for_hook.borrow_mut().push(called_addr);
        true
    });

    emu.regs_mut().rax = NATIVE_MAP_BASE;
    emu.regs_mut().rip = CALLER_IP;
    emu.regs_mut().rsp = STACK_TOP;

    assert!(emu.step());
    assert_eq!(emu.regs().rip, NATIVE_MAP_BASE);
    assert_eq!(emu.regs().rsp, STACK_TOP - 8);
    assert_eq!(
        emu.maps.get_addr_name(NATIVE_MAP_BASE).as_deref(),
        Some("msvcrt.text"),
    );
    assert_eq!(emu.api_addr_to_name(NATIVE_MAP_BASE), "_initterm");
    assert_eq!(
        calls.borrow().as_slice(),
        &[NATIVE_MAP_BASE],
        "on_winapi_call must observe the msvcrt text entry exactly once",
    );
    assert!(
        emu.call_stack().last().is_some(),
        "native gadget must inherit the call-stack frame",
    );
}

#[test]
fn msvcrt_native_executes_fothk_section_bytes() {
    let mut emu = build_msvcrt_native_emu("msvcrtfothk", "_initterm");

    emu.regs_mut().rax = NATIVE_MAP_BASE;
    emu.regs_mut().rip = CALLER_IP;
    emu.regs_mut().rsp = STACK_TOP;

    assert!(emu.step());
    assert_eq!(emu.regs().rip, NATIVE_MAP_BASE);
    assert_eq!(
        emu.maps.get_addr_name(NATIVE_MAP_BASE).as_deref(),
        Some("msvcrtfothk"),
    );
    assert_eq!(emu.regs().rsp, STACK_TOP - 8);
}

#[test]
fn msvcrt_native_unlisted_export_executes_without_fallback_stub() {
    let mut emu = build_msvcrt_native_emu("msvcrt.text", "future_export");

    emu.regs_mut().rax = NATIVE_MAP_BASE;
    emu.regs_mut().rip = CALLER_IP;
    emu.regs_mut().rsp = STACK_TOP;

    assert_eq!(emu.api_addr_to_name(NATIVE_MAP_BASE), "future_export");

    assert!(emu.step());
    assert_eq!(emu.regs().rip, NATIVE_MAP_BASE);
    assert!(!emu.call_stack().is_empty());
}

#[test]
fn msvcrt_native_resolved_iat_thunk_executes_natively() {
    let mut emu = build_msvcrt_native_emu("msvcrt.text", "_initterm");

    let raw = vec![0u8; 0x400];
    let mut pe = rs_header::pe::pe64::PE64::parse("synthetic.dll", &raw);
    let thunk: u64 = 0x0000_DEAD_BEEF;
    pe.iat_names
        .insert(thunk, "msvcrt.dll!_initterm".to_string());
    emu.pe64 = Some(pe);

    emu.regs_mut().rax = thunk;
    emu.regs_mut().rip = CALLER_IP;
    emu.regs_mut().rsp = STACK_TOP;

    assert!(emu.step());
    assert_eq!(emu.regs().rip, NATIVE_MAP_BASE);
    assert_eq!(emu.regs().rsp, STACK_TOP - 8);
    assert!(emu.call_stack().last().is_some());
}

#[test]
fn msvcrt_native_unresolved_msvcrt_dll_thunk_warns_and_skips() {
    use crate::maps::mem64::Permission;

    let mut emu = crate::emu64();
    emu.os = crate::arch::OperatingSystem::Windows;
    emu.cfg.verbose = 0;
    emu.cfg.nocolors = true;

    // Caller + stack so the dispatcher can pop a return address cleanly.
    emu.maps
        .create_map(
            "msvcrt_native_caller",
            CALLER_IP,
            0x400,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("create caller map");
    emu.maps.write_bytes(CALLER_IP, &[0xFF, 0xD0, 0x90, 0x90]);
    emu.maps
        .create_map(
            "msvcrt_native_stack",
            0x8000_0000_0000u64,
            0x1_000,
            Permission::READ_WRITE,
        )
        .expect("create stack map");
    emu.regs_mut().rsp = STACK_TOP;
    emu.regs_mut().rip = CALLER_IP;

    // Synthetic PE64 tagged as msvcrt.dll!_initterm but with no
    // matching export index — the dispatcher must warn and resume at
    // the caller without leaking the RA or call-stack frame.
    let raw = vec![0u8; 0x400];
    let mut pe = rs_header::pe::pe64::PE64::parse("synthetic.dll", &raw);
    let thunk: u64 = 0x0000_DEAD_C0DE;
    pe.iat_names
        .insert(thunk, "msvcrt.dll!_initterm".to_string());
    // PEB/TEB/LDR maps must exist so the resolver's PEB fallback walker
    // can return cleanly when the synthetic msvcrt.dll image is absent.
    emu.maps
        .create_map("peb", 0x7ff7_0000_0000u64, 0x1000, Permission::READ_WRITE)
        .expect("create peb map");
    emu.maps
        .create_map("teb", 0x7ff7_0000_1000u64, 0x1000, Permission::READ_WRITE)
        .expect("create teb map");
    emu.maps
        .create_map("ldr", 0x7ff7_0000_2000u64, 0x1000, Permission::READ_WRITE)
        .expect("create ldr map");
    emu.pe64 = Some(pe);
    emu.regs_mut().rax = thunk;
    assert!(emu.step());
    assert!(emu.call_stack().is_empty());
}
