use crate::exception;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
fn exception_handler64() {
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
    let original_rip = 0x401000;
    let uef_handler = 0x500000;

    emu.regs_mut().rip = original_rip;
    emu.set_uef(uef_handler);

    emu.exception(ExceptionType::Int3);

    assert_eq!(emu.regs().rip, uef_handler);
    assert_ne!(emu.eh_ctx(), 0);

    // Simulate handler returning EXCEPTION_CONTINUE_EXECUTION.
    emu.regs_mut().rax = crate::windows::constants::EXCEPTION_CONTINUE_EXECUTION64;
    exception::handlers::exit64(&mut emu);

    assert_eq!(emu.regs().rip, original_rip);
    assert_eq!(emu.eh_ctx(), 0);
}
