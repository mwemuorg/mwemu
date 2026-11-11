use std::cell::Cell;
use std::rc::Rc;

use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::windows::constants::errors::STATUS_ACCESS_VIOLATION;
use crate::windows::constants::exceptions::*;
use crate::{emu32, emu64, tests::helpers};
use itertools::iproduct;

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
    assert_eq!(emu.fault_count, 1);
}

#[test]
fn run_access_fault_reaches_x86_seh() {
    let mut emu = faulting_emu(false);
    let record = 0x500000;
    emu.maps
        .create_map("seh", record, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_dword(record, EXCEPTION_CHAIN_END);
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

struct ExceptionLayout {
    pointer_size: u64,
    exception_address: u64,
    parameter_count: u64,
    information: u64,
    context_ip: u64,
    context_sp: u64,
    context_flags: u64,
}

impl ExceptionLayout {
    fn new(is_64: bool) -> Self {
        if is_64 {
            Self {
                pointer_size: 8,
                exception_address: 0x10,
                parameter_count: 0x18,
                information: 0x20,
                context_ip: 0xf8,
                context_sp: 0x98,
                context_flags: 0x44,
            }
        } else {
            Self {
                pointer_size: 4,
                exception_address: 0xc,
                parameter_count: 0x10,
                information: 0x14,
                context_ip: 0xb8,
                context_sp: 0xc4,
                context_flags: 0xc0,
            }
        }
    }

    fn pointer(&self, emu: &Emu, address: u64) -> u64 {
        if self.pointer_size == 8 {
            emu.maps.read_qword(address)
        } else {
            emu.maps.read_dword(address).map(u64::from)
        }
        .expect("readable guest pointer")
    }

    fn write_pointer(&self, emu: &mut Emu, address: u64, value: u64) {
        assert!(if self.pointer_size == 8 {
            emu.maps.write_qword(address, value)
        } else {
            emu.maps.write_dword(address, value as u32)
        });
    }

    fn callback_records(&self, emu: &Emu) -> (u64, u64) {
        let pointers = if emu.cfg.is_x64() {
            emu.regs().rcx
        } else {
            self.pointer(emu, emu.sp() + self.pointer_size)
        };
        (
            self.pointer(emu, pointers),
            self.pointer(emu, pointers + self.pointer_size),
        )
    }

    fn assert_access_violation(&self, emu: &Emu, record: u64, pc: u64, access: u64, address: u64) {
        assert_eq!(
            emu.maps.read_dword(record),
            Some(STATUS_ACCESS_VIOLATION as u32)
        );
        assert_eq!(self.pointer(emu, record + self.exception_address), pc);
        assert_eq!(emu.maps.read_dword(record + self.parameter_count), Some(2));
        assert_eq!(self.pointer(emu, record + self.information), access);
        assert_eq!(
            self.pointer(emu, record + self.information + self.pointer_size),
            address
        );
    }
}

#[test]
fn cpu_fault_callbacks_receive_exception_pointers() {
    const FAULT_ADDRESS: u64 = 0xdeadbeef;
    for (is_64, use_veh) in iproduct!([false, true], [false, true]) {
        let mut emu = faulting_emu(is_64);
        let layout = ExceptionLayout::new(is_64);
        emu.regs_mut().rax = FAULT_ADDRESS;
        let sp = emu.sp();
        if use_veh {
            emu.set_veh(HANDLER);
        } else {
            emu.set_uef(HANDLER);
        }
        assert!(!emu.step());
        assert_eq!(emu.pc(), HANDLER);
        let (record, context) = layout.callback_records(&emu);
        layout.assert_access_violation(&emu, record, CODE, EXCEPTION_WRITE_FAULT, FAULT_ADDRESS);
        assert_eq!(layout.pointer(&emu, context + layout.context_sp), sp);
        if is_64 {
            assert_eq!(emu.sp() % 16, 8, "x64 callback stack alignment");
            assert!(
                emu.maps
                    .can_access_range(emu.sp() + 8, 32, Permission::WRITE),
                "x64 shadow space"
            );
        }
    }
}

#[test]
fn cpu_fault_callback_helper_return_keeps_exception_active() {
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        emu.set_uef(HANDLER);
        emu.maps.write_bytes(
            HANDLER,
            &[
                0xe8, 0x06, 0, 0, 0, // call helper
                0xb8, 0xff, 0xff, 0xff, 0xff, // EXCEPTION_CONTINUE_EXECUTION
                0xc3, // handler return
                0x90, 0xc3, // helper: nop; ret
            ],
        );
        assert!(!emu.step());
        let context = emu.eh_ctx();
        for _ in 0..3 {
            assert!(emu.step());
        }
        assert_eq!(emu.pc(), HANDLER + 5, "x64={is_64}");
        assert_eq!(emu.eh_ctx(), context);
    }
}

#[test]
fn cpu_fault_veh_continue_search_advances_to_next_handler() {
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        let next = HANDLER + 0x100;
        emu.add_veh(false, HANDLER);
        emu.add_veh(false, next);
        emu.maps.write_bytes(HANDLER, &[0x31, 0xc0, 0xc3]); // continue search
        assert!(!emu.step());
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.pc(), next, "x64={is_64}");
        assert_eq!(emu.fault_count, 1);
    }
}

#[test]
fn access_faults_report_read_write_and_execute() {
    const FAULT_ADDRESS: u64 = 0xdeadbeef;
    let cases: &[(&str, &[u8], u64)] = &[
        ("scalar read", &[0x8b, 0x08], EXCEPTION_READ_FAULT), // mov ecx, [eax/rax]
        ("scalar write", &[0x89, 0x08], EXCEPTION_WRITE_FAULT), // mov [eax/rax], ecx
        ("SIMD read", &[0x0f, 0x10, 0x00], EXCEPTION_READ_FAULT), // movups xmm0, [eax/rax]
        ("SIMD write", &[0x0f, 0x11, 0x00], EXCEPTION_WRITE_FAULT), // movups [eax/rax], xmm0
        ("unmapped jump", &[0xff, 0xe0], EXCEPTION_EXECUTE_FAULT), // jmp eax/rax
    ];
    for (is_64, &(name, code, access)) in iproduct!([false, true], cases) {
        let mut emu = faulting_emu(is_64);
        let layout = ExceptionLayout::new(is_64);
        emu.set_uef(HANDLER);
        emu.regs_mut().rax = FAULT_ADDRESS;
        assert!(emu.maps.write_bytes(CODE, code));
        assert!(!emu.step(), "{name}, x64={is_64}");
        assert_eq!(emu.pc(), HANDLER, "{name}, x64={is_64}");
        let (record, context) = layout.callback_records(&emu);
        let pc = if access == EXCEPTION_EXECUTE_FAULT {
            FAULT_ADDRESS
        } else {
            CODE
        };
        layout.assert_access_violation(&emu, record, pc, access, FAULT_ADDRESS);
        assert_eq!(layout.pointer(&emu, context + layout.context_ip), pc);
        assert_eq!(emu.fault_count, 1);
    }
}

#[test]
fn instruction_fetch_fault_reaches_handler_in_step_and_run() {
    const INACCESSIBLE: u64 = 0x500000;
    for (is_64, cached, mapped) in iproduct!([false, true], [false, true], [false, true]) {
        let mut emu = faulting_emu(is_64);
        let layout = ExceptionLayout::new(is_64);
        if mapped {
            emu.maps
                .create_map("data", INACCESSIBLE, 0x1000, Permission::NONE)
                .unwrap();
        }
        emu.regs_mut().rip = INACCESSIBLE;
        emu.set_uef(HANDLER);
        if cached {
            assert_eq!(emu.run(Some(HANDLER_END)).unwrap(), HANDLER_END);
        } else {
            assert!(!emu.step());
            assert_eq!(emu.pc(), HANDLER);
        }
        let (record, _) = layout.callback_records(&emu);
        layout.assert_access_violation(
            &emu,
            record,
            INACCESSIBLE,
            EXCEPTION_EXECUTE_FAULT,
            INACCESSIBLE,
        );
        assert_eq!(emu.fault_count, 1);
    }
}

#[test]
fn callback_resumes_edited_context_and_flags() {
    const RESUMED_FLAGS: u32 = 0x246;
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        let layout = ExceptionLayout::new(is_64);
        let sp = emu.sp();
        emu.set_uef(HANDLER);
        assert!(
            emu.maps
                .write_bytes(HANDLER, &[0xb8, 0xff, 0xff, 0xff, 0xff, 0xc3])
        ); // continue execution
        assert!(!emu.step());
        let (record, context) = layout.callback_records(&emu);
        layout.write_pointer(&mut emu, context + layout.context_ip, CODE + 2);
        assert!(
            emu.maps
                .write_dword(context + layout.context_flags, RESUMED_FLAGS)
        );
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.pc(), CODE + 2);
        assert_eq!(emu.sp(), sp);
        assert_eq!(emu.flags().dump(), RESUMED_FLAGS);
        assert_eq!(emu.eh_ctx(), 0);
        assert!(!emu.maps.is_mapped(record));
    }
}

#[test]
fn veh_search_falls_through_to_seh_then_uef_without_losing_record() {
    const REGISTRATION: u64 = 0x500000;
    const SEH_HANDLER: u64 = HANDLER + 0x100;
    const UEF_HANDLER: u64 = HANDLER + 0x200;
    const FAULT_ADDRESS: u64 = 0xdeadbeef;
    let mut emu = faulting_emu(false);
    let layout = ExceptionLayout::new(false);
    emu.maps
        .create_map("seh", REGISTRATION, 0x1000, Permission::READ_WRITE)
        .unwrap();
    assert!(emu.maps.write_dword(REGISTRATION, EXCEPTION_CHAIN_END));
    assert!(emu.maps.write_dword(REGISTRATION + 4, SEH_HANDLER as u32));
    emu.set_seh(REGISTRATION);
    emu.set_veh(HANDLER);
    emu.set_uef(UEF_HANDLER);
    emu.regs_mut().rax = FAULT_ADDRESS;
    assert!(emu.maps.write_bytes(HANDLER, &[0x31, 0xc0, 0xc3])); // VEH ContinueSearch=0
    assert!(emu.maps.write_bytes(SEH_HANDLER, &[0xb8, 1, 0, 0, 0, 0xc3])); // SEH ContinueSearch=1
    assert!(emu.maps.write_bytes(UEF_HANDLER, &[0x31, 0xc0, 0xc3])); // UEF ContinueSearch terminates
    assert!(!emu.step());
    assert!(emu.step());
    assert!(emu.step());
    assert_eq!(emu.pc(), SEH_HANDLER);
    let seh_record = layout.pointer(&emu, emu.sp() + layout.pointer_size);
    layout.assert_access_violation(&emu, seh_record, CODE, EXCEPTION_WRITE_FAULT, FAULT_ADDRESS);
    assert!(emu.step());
    assert!(emu.step());
    assert_eq!(emu.pc(), UEF_HANDLER);
    let (record, _) = layout.callback_records(&emu);
    layout.assert_access_violation(&emu, record, CODE, EXCEPTION_WRITE_FAULT, FAULT_ADDRESS);
    assert!(emu.step());
    assert!(emu.step());
    assert!(emu.process_terminated);
    assert_eq!(emu.eh_ctx(), 0);
    assert_eq!(emu.fault_count, 1);
}

#[test]
fn cancelled_exception_does_not_terminate_without_handlers() {
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        emu.set_hook_on_exception(|_, _, _| false);
        emu.exception(ExceptionType::Int3);
        assert!(!emu.process_terminated);
        assert_eq!(emu.eh_ctx(), 0);
        assert_eq!(emu.pc(), CODE);
    }
}

#[test]
fn cross_boundary_fault_reports_first_inaccessible_byte() {
    const DATA: u64 = 0x500000;
    const REGION_SIZE: u64 = 8;
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        let layout = ExceptionLayout::new(is_64);
        emu.maps
            .create_map("writable", DATA, REGION_SIZE, Permission::READ_WRITE)
            .unwrap();
        emu.maps
            .create_map(
                "readonly",
                DATA + REGION_SIZE,
                REGION_SIZE,
                Permission::READ,
            )
            .unwrap();
        emu.set_uef(HANDLER);
        emu.regs_mut().rax = DATA;
        assert!(emu.maps.write_bytes(CODE, &[0x0f, 0x11, 0x00])); // movups [eax/rax], xmm0
        assert!(!emu.step());
        let (record, _) = layout.callback_records(&emu);
        layout.assert_access_violation(
            &emu,
            record,
            CODE,
            EXCEPTION_WRITE_FAULT,
            DATA + REGION_SIZE,
        );
    }
}

#[test]
fn nested_callback_restores_outer_dispatch_frame() {
    const NESTED_RECORD: u64 = 8;
    const INNER_HANDLER: u64 = HANDLER + 0x100;
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        let layout = ExceptionLayout::new(is_64);
        emu.set_uef(HANDLER);
        assert!(emu.maps.write_bytes(
            HANDLER,
            &[
                0x89, 0x08, // mov [eax/rax], ecx: nested fault
                0xb8, 0xff, 0xff, 0xff, 0xff, 0xc3, // continue execution
            ]
        ));
        assert!(
            emu.maps
                .write_bytes(INNER_HANDLER, &[0xb8, 0xff, 0xff, 0xff, 0xff, 0xc3])
        );
        assert!(!emu.step());
        let (outer_record, outer_context) = layout.callback_records(&emu);
        let outer_sp = emu.sp();
        emu.set_uef(INNER_HANDLER);
        assert!(!emu.step());
        assert_eq!(emu.pc(), INNER_HANDLER);
        let (inner_record, inner_context) = layout.callback_records(&emu);
        assert_ne!(outer_context, inner_context);
        assert_eq!(
            layout.pointer(&emu, inner_record + NESTED_RECORD),
            outer_record
        );
        layout.write_pointer(&mut emu, inner_context + layout.context_ip, HANDLER + 2);
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.eh_ctx(), outer_context);
        assert_eq!(emu.sp(), outer_sp);
        assert_eq!(emu.pc(), HANDLER + 2);
        assert!(emu.maps.is_mapped(outer_record));
        assert!(!emu.maps.is_mapped(inner_record));
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.eh_ctx(), 0);
        assert_eq!(emu.pc(), CODE);
        assert!(!emu.maps.is_mapped(outer_record));
    }
}

#[test]
#[allow(deprecated)]
fn legacy_step_fetch_fault_executes_first_handler_instruction_once() {
    const INACCESSIBLE: u64 = 0x500000;
    for is_64 in [false, true] {
        let mut emu = faulting_emu(is_64);
        emu.set_uef(HANDLER);
        emu.regs_mut().rip = INACCESSIBLE;
        assert!(!emu.step_single_threaded());
        assert_eq!(emu.pc(), HANDLER);
        assert!(emu.step_single_threaded());
        assert_eq!(emu.pc(), HANDLER_END);
        assert_eq!(emu.regs().rbx, MARKER);
    }
}
