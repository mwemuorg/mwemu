use crate::{tests::helpers, *};

#[test]
// this tests windows 32bits shellcodes, and fetching apis and doing some api calls, pointing
// to strings etc.
pub fn sc32win_peb_ldr_rot() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();

    let sample = helpers::test_data_path("sc32win_peb_ldr_rot.bin");
    emu.load_code(&sample);
    emu.run(Some(0x3c0116));

    let ptr = emu.regs().get_ebx();
    assert_eq!(ptr, 0x3c01b8);
    let s: String = emu.maps.read_string(ptr);
    assert!(s.starts_with("Host: msn.com"));
}
