use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
// this test a windows 64bits executable that calculates apis like shellcodes and does basic api calls.
// aso read strings and patch string.
pub fn exe64win_msgbox() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    emu.load_code(&sample);
    emu.run(Some(0x14000123f));

    let message = emu.maps.read_string(emu.regs().rdx);
    let title = emu.maps.read_string(emu.regs().rdi);

    assert_eq!(message, "message");
    assert_eq!(title, "title");

    // we need to set the permission to use it
    let mem = emu
        .maps
        .get_mem_by_addr_mut(emu.regs().rdx)
        .expect("the memory need to be there");
    mem.set_permission(Permission::READ_WRITE);
    emu.maps.write_string(emu.regs().rdx, "inject");

    // launch the msgbox
    emu.step();
}
