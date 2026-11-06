use std::cell::Cell;
use std::rc::Rc;

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::{emu32, emu64, tests::helpers};

const CODE: u64 = 0x400000;
const HANDLER: u64 = CODE + 0x100;
const HANDLER_END: u64 = HANDLER + 5;
const MARKER: u64 = 0x12345678;

fn faulting_emu(is_64: bool) -> Emu {
    helpers::setup();
    let mut emu = if is_64 { emu64() } else { emu32() };
    emu.cfg.console_enabled = false;
    emu.cfg.soft_exception_recovery = false;
    emu.cfg.max_instructions = Some(16);
    emu.running_script = true;
    if is_64 {
        emu.init_stack64();
    } else {
        emu.init_stack32();
    }
    emu.maps
        .create_map("code", CODE, 0x1000, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    // mov [eax/rax], ecx faults at address zero. The following mov must not run.
    emu.maps
        .write_bytes(CODE, &[0x89, 0x08, 0xbb, 0xef, 0xbe, 0xad, 0xde]);
    // The handler's first instruction writes an observable marker to EBX.
    emu.maps
        .write_bytes(HANDLER, &[0xbb, 0x78, 0x56, 0x34, 0x12, 0x90]);
    emu.regs_mut().rip = CODE;
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 0;
    emu
}

fn assert_handler_runs(emu: &mut Emu) {
    let observed_failure = Rc::new(Cell::new(false));
    let observed = observed_failure.clone();
    emu.set_hook_on_post_instruction(move |_, _, _, _, ok| {
        if !ok {
            observed.set(true);
        }
    });
    assert_eq!(emu.run(Some(HANDLER_END)).unwrap(), HANDLER_END);
    assert_eq!(emu.regs().rbx, MARKER);
    assert_eq!(
        emu.pos, 2,
        "one faulting instruction and one handler instruction"
    );
    assert!(
        observed_failure.get(),
        "the fault must remain visible to hooks"
    );
    assert!(!emu.process_terminated);
    assert!(!emu.force_reload);
}

#[test]
fn run_access_fault_reaches_x86_seh() {
    let mut emu = faulting_emu(false);
    let record = 0x500000;
    emu.maps
        .create_map("seh", record, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_dword(record, u32::MAX);
    emu.maps.write_dword(record + 4, HANDLER as u32);
    emu.set_seh(record);
    assert_handler_runs(&mut emu);
}

#[test]
fn run_access_fault_reaches_veh_and_uef() {
    for is_64 in [false, true] {
        for use_veh in [false, true] {
            let mut emu = faulting_emu(is_64);
            if use_veh {
                emu.set_veh(HANDLER);
            } else {
                emu.set_uef(HANDLER);
            }
            assert_handler_runs(&mut emu);
        }
    }
}

#[test]
fn run_access_fault_without_handler_stops() {
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        assert_eq!(emu.run(Some(HANDLER_END)).unwrap(), CODE);
        assert_eq!(emu.pos, 1);
        assert_eq!(emu.regs().rbx, 0);
        assert!(emu.process_terminated);
    }
}

#[test]
fn run_access_fault_in_handler_without_redirect_stops() {
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        emu.set_uef(HANDLER);
        emu.maps.write_bytes(HANDLER, &[0x89, 0x08]);
        let exceptions = Rc::new(Cell::new(0));
        let observed = exceptions.clone();
        emu.set_hook_on_exception(move |_, _, _| {
            observed.set(observed.get() + 1);
            observed.get() == 1 // Deliver the first exception, cancel the second.
        });
        assert_eq!(emu.run(Some(HANDLER_END)).unwrap(), HANDLER);
        assert_eq!(exceptions.get(), 2);
        assert_eq!(emu.pos, 2);
        assert_ne!(emu.eh_ctx(), 0);
        assert_eq!(emu.regs().rbx, 0);
    }
}

#[test]
fn run_access_fault_with_unmapped_x64_handler_stops() {
    let mut emu = faulting_emu(true);
    emu.set_uef(0x900000);
    // A failed handler jump can leave force_reload and a context set without
    // changing PC. It must not cause a retry of the faulting instruction.
    assert_eq!(emu.run(Some(HANDLER_END)).unwrap(), CODE);
    assert_eq!(emu.pos, 1);
    assert_eq!(emu.regs().rbx, 0);
}
