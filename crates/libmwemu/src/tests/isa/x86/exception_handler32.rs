use crate::tests::helpers;
use crate::*;

#[test]
// context objects for exception recovering
pub fn exception_handler32() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = win_maps!(32);
    emu.load_code(&sample!("exe32win_exception_handler.bin"));
}
