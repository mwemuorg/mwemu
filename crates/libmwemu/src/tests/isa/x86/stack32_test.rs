use crate::{tests::helpers, *};

#[test]
// stack32 tests
pub fn stack32_test() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    emu.init_win32(false, false);

    let stack_check = emu.maps.get_map_by_name("stack");
    assert!(stack_check.is_some());
    let stack = stack_check.unwrap();
    let base = stack.get_base();

    assert!(emu.regs().get_esp() < emu.regs().get_ebp());
    assert!(emu.regs().get_esp() > stack.get_base());
    assert!(emu.regs().get_esp() < stack.get_bottom());
    assert!(emu.regs().get_ebp() > stack.get_base());
    assert!(emu.regs().get_ebp() < stack.get_bottom());
    assert!(stack.inside(emu.regs().get_esp()));
    assert!(stack.inside(emu.regs().get_ebp()));

    for i in 0..5000 {
        emu.stack_push32(i as u32);
    }
    emu.stack_pop32(false);

    assert!(emu.regs().get_esp() > base);
}

#[test]
fn initial_test_stack_alignment_32bit_bare_metal() {
    let mut emu = emu32();
    emu.init_cpu();

    assert_eq!(
        emu.regs().get_esp() % 4,
        0,
        "in 32bits stack has to be aligned to 4 bytes"
    );
}
