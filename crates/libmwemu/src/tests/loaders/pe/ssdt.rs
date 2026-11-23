use crate::context::context64::Context64;
use crate::tests::helpers;
use crate::winapi::policy::ApiMode;
use crate::windows::constants::process::PROCESS_TLS_INFORMATION;
use crate::windows::constants::syscalls::{WIN64_NTCONTINUE, WIN64_NTSETINFORMATIONPROCESS};
use crate::windows::constants::system::IDOK;
use crate::windows::peb::peb64;
use crate::windows::structures::{RtlUserProcessParameters64, TEB64, UnicodeString64};
use crate::*;
use goblin::pe::PE;
use std::cell::Cell;
use std::fs;
use std::rc::Rc;

fn ssdt_emu(mode: ApiMode) -> Emu {
    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::system_dlls64();
    emu.cfg.winver = None;
    emu.cfg.emulate_winapi = true;
    emu.set_api_mode(mode);
    emu
}

#[test]
fn win11_loader_reads_the_relocated_executable_path_in_strict_mode() {
    const SCAN_LIMIT: u64 = 1_000_000;
    helpers::setup();
    let mut emu = ssdt_emu(ApiMode::Strict);
    let relocated = Rc::new(Cell::new(false));
    let observed = Rc::clone(&relocated);
    let mut original = None;
    emu.hooks.on_syscall(move |emu, _| {
        if let Some(current) = peb64::current_parameters(&emu.maps) {
            let first = *original.get_or_insert(current);
            if current != first {
                let path = UnicodeString64::load(
                    current + RtlUserProcessParameters64::IMAGE_PATH_NAME_OFFSET,
                    &emu.maps,
                );
                if emu
                    .maps
                    .read_wide_string(path.buffer)
                    .ends_with("exe64win_msgbox.bin")
                {
                    observed.set(true);
                    emu.stop();
                }
            }
        }
        true
    });
    emu.hooks.on_pre_instruction(|emu, _, _, _| {
        if emu.pos >= SCAN_LIMIT {
            emu.stop();
            return false;
        }
        true
    });
    let result = emu.load_code(&sample!("exe64win_msgbox.bin"));
    assert!(
        relocated.get(),
        "loader did not publish the executable path at pos {}: {result:?}",
        emu.pos
    );
}

fn run_strict_ssdt_to_entry(name: &str, extra: u64) {
    helpers::setup();
    let path = sample!(name);
    let bytes = fs::read(&path).expect("read the EXE fixture");
    let entry_rva = PE::parse(&bytes).expect("read the EXE entry point").entry as u64;
    let mut emu = ssdt_emu(ApiMode::Strict);

    let continued = Rc::new(Cell::new(false));
    let observed = Rc::clone(&continued);
    emu.hooks.on_syscall(move |emu, number| {
        if number == WIN64_NTCONTINUE
            && emu.maps.read_qword(emu.regs().rcx + Context64::RIP_OFFSET)
                == Some(emu.base + entry_rva)
        {
            observed.set(true);
        }
        true
    });
    let at_entry = Rc::new(Cell::new(false));
    let pending = Rc::clone(&at_entry);
    let retired = Rc::new(Cell::new(None));
    let observation = Rc::clone(&retired);
    let resumed = Rc::clone(&continued);
    emu.hooks.on_pre_instruction(move |emu, pc, _, _| {
        const LOADER_LIMIT: u64 = 24_000_000;
        if emu.pos >= LOADER_LIMIT {
            emu.stop();
            return false;
        }
        pending.set(resumed.get() && pc == emu.base + entry_rva);
        true
    });
    emu.hooks
        .on_post_instruction(move |emu, _, _, _, executed| {
            if at_entry.replace(false) && executed && observation.get().is_none() {
                observation.set(Some(emu.pos));
            }
        });

    emu.load_code(&path)
        .unwrap_or_else(|error| panic!("{name} at {}: {error}", emu.pos));
    assert!(
        continued.get() && emu.ldr_init_done,
        "{name}: loader never resumed the EXE at instruction {}",
        emu.pos
    );
    let target = emu.pos + extra;
    emu.run_to(target)
        .unwrap_or_else(|error| panic!("{name} at {}: {error}", emu.pos));
    assert!(
        retired.get().is_some(),
        "{name}: first EXE instruction did not execute"
    );
    assert!(
        !emu.process_terminated && emu.pos >= target,
        "{name} stopped at {}",
        emu.pos
    );
}

#[test]
fn enigma_ssdt_installs_the_tls_vector_in_the_teb() {
    helpers::setup();
    let mut emu = ssdt_emu(ApiMode::Strict);
    let request = Rc::new(Cell::new(None));
    let pending = Rc::clone(&request);
    let installed = Rc::new(Cell::new(false));
    let observed = Rc::clone(&installed);
    emu.hooks.on_syscall(move |emu, number| {
        if number == WIN64_NTSETINFORMATIONPROCESS
            && emu.regs().rdx == PROCESS_TLS_INFORMATION as u64
        {
            let buffer = emu.regs().r8;
            if emu.maps.read_dword(buffer + 4) == Some(1) {
                pending.set(
                    emu.maps
                        .read_qword(buffer + 24)
                        .map(|vector| (buffer, vector)),
                );
            }
        }
        true
    });
    emu.hooks.on_pre_instruction(move |emu, _, _, _| {
        if let Some((buffer, vector)) = request.take() {
            let teb = emu.windows_teb().expect("current Windows thread");
            observed.set(
                vector != 0
                    && emu.regs().rax == 0
                    && emu.maps.read_dword(buffer + 16) == Some(2)
                    && emu
                        .maps
                        .read_qword(teb + TEB64::THREAD_LOCAL_STORAGE_POINTER_OFFSET)
                        == Some(vector),
            );
            emu.stop();
            return false;
        }
        if emu.pos >= 6_000_000 {
            emu.stop();
            return false;
        }
        true
    });
    let result = emu.load_code(&sample!("exe64win_enigma.bin"));
    assert!(
        installed.get(),
        "guest TLS vector at {}: {result:?}",
        emu.pos
    );
}

#[test]
fn exe64win_enigma_ssdt_runs_deep() {
    const TARGET: u64 = 50_000_000;
    helpers::setup();
    let mut emu = ssdt_emu(ApiMode::Strict);

    emu.load_code(&sample!("exe64win_enigma.bin"))
        .expect("initialize the Enigma process");
    emu.run_to(TARGET).expect("run the Enigma process");
    assert!(
        emu.pos >= TARGET && !emu.process_terminated,
        "Enigma stopped at instruction {}, RIP {:#x}",
        emu.pos,
        emu.pc()
    );
}

#[test]
fn win11_ssdt_executes_messagebox_entry() {
    run_strict_ssdt_to_entry("exe64win_msgbox.bin", 120);
}

#[test]
fn win11_ssdt_executes_mingw_entry() {
    run_strict_ssdt_to_entry("exe64win_mingw.bin", 120);
}

#[test]
fn win11_ssdt_executes_enigma_entry() {
    run_strict_ssdt_to_entry("exe64win_enigma.bin", 500_000);
}

#[test]
fn ssdt_ldr_initialize_thunk() {
    const MIN_LOADER_INSTRUCTIONS: u64 = 15_000;
    helpers::setup();
    let mut emu = ssdt_emu(ApiMode::SimulateSuccess);

    let path = sample!("exe64win_msgbox.bin");
    let bytes = fs::read(&path).expect("read the MessageBox fixture");
    let entry_rva = PE::parse(&bytes)
        .expect("parse the MessageBox fixture")
        .entry as u64;
    emu.load_code(&path)
        .expect("initialize the MessageBox process");
    assert!(emu.pos > MIN_LOADER_INSTRUCTIONS && emu.ldr_init_done && !emu.process_terminated);
    assert_eq!(
        emu.pc(),
        emu.base + entry_rva,
        "resume at the MessageBox entry point"
    );
}

#[test]
fn ssdt_msgbox_reaches_messageboxa() {
    helpers::setup();
    let mut emu = ssdt_emu(ApiMode::Strict);
    emu.cfg.max_instructions = Some(12_000_000);
    emu.load_code(&sample!("exe64win_msgbox.bin"))
        .expect("load MessageBoxA sample");

    assert!(
        emu.ldr_init_done,
        "ntdll!LdrInitializeThunk did not complete (pos={} rip=0x{:x})",
        emu.pos,
        emu.regs().rip
    );

    // The MessageBoxA shim can return before the first instruction in User32 is decoded.
    let return_pc = emu.base + helpers::MSGBOX64_RETURN_RVA;
    let stopped_at = emu.run(Some(return_pc)).expect("run to MessageBoxA return");
    assert!(
        emu.maps.get_map_by_name("user32.pe").is_some(),
        "user32.dll was never mapped (pos={} rip={:#x})",
        emu.pos,
        emu.pc()
    );
    assert_eq!(
        stopped_at, return_pc,
        "EXE stopped before MessageBoxA returned at instruction {}",
        emu.pos
    );
    assert_eq!(emu.regs().rax, IDOK, "MessageBoxA result");
}
