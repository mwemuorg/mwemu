use crate::tests::helpers;
use crate::*;

#[test]
// this tests a linux 64bits flags
pub fn elf64lin_flags() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = helpers::test_data_path("elf64lin_flags.bin");
    emu.load_code(&sample);

    // test instruction add
    emu.run(Some(0x401014));
    assert_eq!(emu.flags().f_cf, true);
    assert_eq!(emu.flags().f_of, false);
    assert_eq!(emu.flags().f_zf, true);
    assert_eq!(emu.flags().f_sf, false);
    assert_eq!(emu.flags().f_pf, true);

    // test instruction sub
    emu.run(Some(0x40102a));
    assert_eq!(emu.flags().f_cf, false);
    assert_eq!(emu.flags().f_of, false);
    assert_eq!(emu.flags().f_zf, true);
    assert_eq!(emu.flags().f_sf, false);
    assert_eq!(emu.flags().f_pf, true);

    // test instruction cmp
    emu.run(Some(0x401040));
    assert_eq!(emu.flags().f_cf, true);
    assert_eq!(emu.flags().f_of, false);
    assert_eq!(emu.flags().f_zf, false);
    assert_eq!(emu.flags().f_sf, true);
    assert_eq!(emu.flags().f_pf, false);

    // test instruction test
    emu.run(Some(0x401056));
    assert_eq!(emu.flags().f_cf, false);
    assert_eq!(emu.flags().f_of, false);
    assert_eq!(emu.flags().f_zf, true);
    assert_eq!(emu.flags().f_sf, false);
    assert_eq!(emu.flags().f_pf, true);

    // test and
    emu.run(Some(0x40106c));
    assert_eq!(emu.flags().f_cf, false);
    assert_eq!(emu.flags().f_of, false);
    assert_eq!(emu.flags().f_zf, true);
    assert_eq!(emu.flags().f_sf, false);
    assert_eq!(emu.flags().f_pf, true);

    // test or with 0x0
    emu.run(Some(0x401087));
    assert_eq!(emu.flags().f_cf, false);
    assert_eq!(emu.flags().f_of, false);
    assert_eq!(emu.flags().f_zf, false);
    assert_eq!(emu.flags().f_sf, true);
    assert_eq!(emu.flags().f_pf, true);

    // test shl
    emu.run(Some(0x40109d));
    assert_eq!(emu.flags().f_cf, true);
    assert_eq!(emu.flags().f_of, true);
    assert_eq!(emu.flags().f_zf, true);
    assert_eq!(emu.flags().f_sf, false);
    assert_eq!(emu.flags().f_pf, true);

    // test add
    emu.run(Some(0x4010b8));
    assert_eq!(emu.flags().f_cf, false);
    assert_eq!(emu.flags().f_of, true);
    assert_eq!(emu.flags().f_zf, false);
    assert_eq!(emu.flags().f_sf, true);
    assert_eq!(emu.flags().f_pf, true);
}
