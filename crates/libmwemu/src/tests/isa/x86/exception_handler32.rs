use crate::tests::helpers;
use crate::*;

#[test]
// context objects for exception recovering
pub fn exception_handler32() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    emu.load_code(&helpers::test_data_path("exe32win_exception_handler.bin"));
}
