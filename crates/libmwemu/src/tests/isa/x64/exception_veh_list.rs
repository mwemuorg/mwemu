use crate::exception;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
fn veh_multiple_handlers_first_param() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();
    emu.maps
        .create_map(
            "test_code",
            0x400000,
            0x200000,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("cannot create test code map");

    let handler_a = 0x500000u64;
    let handler_b = 0x500100u64;
    let handler_c = 0x500200u64;

    emu.add_veh(false, handler_a);
    emu.add_veh(false, handler_b);
    emu.add_veh(true, handler_c);

    assert_eq!(emu.veh_list(), &[handler_c, handler_a, handler_b]);
    assert_eq!(emu.veh(), handler_c);
}

#[test]
fn veh_remove_handler() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();
    emu.maps
        .create_map(
            "test_code",
            0x400000,
            0x200000,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("cannot create test code map");

    let handler_a = 0x500000u64;
    let handler_b = 0x500100u64;

    emu.add_veh(false, handler_a);
    emu.add_veh(false, handler_b);
    assert_eq!(emu.veh_list(), &[handler_a, handler_b]);

    emu.remove_veh(handler_a);
    assert_eq!(emu.veh_list(), &[handler_b]);
    assert_eq!(emu.veh(), handler_b);

    emu.remove_veh(handler_b);
    assert_eq!(emu.veh(), 0);
    assert!(emu.veh_list().is_empty());
}

#[test]
fn veh_dispatches_first_handler() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();
    emu.maps
        .create_map(
            "test_code",
            0x400000,
            0x200000,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("cannot create test code map");

    let original_rip = 0x401000u64;
    let handler_a = 0x500000u64;
    let handler_b = 0x500100u64;

    emu.regs_mut().rip = original_rip;
    emu.add_veh(false, handler_a);
    emu.add_veh(false, handler_b);

    emu.exception(ExceptionType::Int3);

    assert_eq!(emu.regs().rip, handler_a);
    assert_ne!(emu.eh_ctx(), 0);

    emu.regs_mut().rax = crate::windows::constants::EXCEPTION_CONTINUE_EXECUTION64;
    exception::handlers::exit64(&mut emu);

    assert_eq!(emu.regs().rip, original_rip);
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn veh_set_clears_list() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();
    emu.maps
        .create_map(
            "test_code",
            0x400000,
            0x200000,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("cannot create test code map");

    emu.add_veh(false, 0x500000);
    emu.add_veh(false, 0x500100);
    assert_eq!(emu.veh_list().len(), 2);

    emu.set_veh(0x600000);
    assert_eq!(emu.veh_list(), &[0x600000u64]);

    emu.set_veh(0);
    assert!(emu.veh_list().is_empty());
}
