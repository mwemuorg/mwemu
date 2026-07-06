use crate::tests::helpers;
use crate::*;

#[test]
// build string: cargo run --release -- -6 -f test/sc64lin_strgen.bin -vv -c 232 -s 0x329ec8
pub fn sc64win_strgen() {
    helpers::setup();

    let mut emu = emu64();
    emu.set_verbose(3);
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.load_code(&helpers::test_data_path("sc64win_strgen.bin"));
    emu.run_to(231);
    let s = emu.maps.read_string(0x329ec8);
    assert_eq!(s, "http://something.com/");
}
