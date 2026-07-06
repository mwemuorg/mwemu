use crate::tests::helpers;
use crate::*;

#[test]
// this tests a windows 32bits executable, that require iat binding of multiple libs.
pub fn exe32win_minecraft() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();

    let sample = helpers::test_data_path("exe32win_minecraft.bin");
    emu.load_code(&sample);
    emu.run(Some(0x403740));

    assert_eq!(emu.regs().get_ebx(), 2);
}
