use crate::{tests::helpers, *};

#[test]
// stack64 tests
pub fn stack64_test() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.init_win32(false, false);

    let stack_check = emu.maps.get_map_by_name("stack");
    assert!(stack_check.is_some());
    let stack = stack_check.unwrap();
    let base = stack.get_base();

    assert!(emu.regs().rsp < emu.regs().rbp);
    assert!(emu.regs().rsp > stack.get_base());
    assert!(emu.regs().rsp < stack.get_bottom());
    assert!(emu.regs().rbp > stack.get_base());
    assert!(emu.regs().rbp < stack.get_bottom());
    assert!(stack.inside(emu.regs().rsp));
    assert!(stack.inside(emu.regs().rbp));

    for i in 0..5000 {
        emu.stack_push64(i as u64);
    }
    emu.stack_pop64(false);

    assert!(emu.regs().rsp > base);
}

#[test]
fn initial_test_stack_alignment_bare_metal() {
    let mut emu = emu64();
    emu.init_cpu();
    assert_eq!(
        emu.regs().rsp % 16,
        0,
        "64bits stack has to be aligned to 16"
    );
}
