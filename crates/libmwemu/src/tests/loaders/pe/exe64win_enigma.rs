use crate::tests::helpers;
use crate::*;

#[test]
// enigma packer should be emulated at least 102,302,404 insturctions.
// this test is few seconds slow but will verify many cpu instructions.
pub fn exe64win_enigma() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = helpers::test_data_path("exe64win_enigma.bin");
    emu.load_code(&sample);
    emu.run_to(5_000_000 + 5);
    assert!(emu.pos >= 5_000_000);
    /*
    too slow for a test:
    emu.run_to(102072786);
    assert!(emu.pos > 102072780);
    */
}
