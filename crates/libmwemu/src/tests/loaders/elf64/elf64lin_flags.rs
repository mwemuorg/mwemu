use crate::tests::helpers;
use crate::*;

#[test]
// this tests a linux 64bits flags
pub fn elf64lin_flags() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = sample!("elf64lin_flags.bin");
    emu.load_code(&sample);

    // test instruction add
    emu.run(Some(0x401014));
    assert!(emu.flags().f_cf);
    assert!(!emu.flags().f_of);
    assert!(emu.flags().f_zf);
    assert!(!emu.flags().f_sf);
    assert!(emu.flags().f_pf);

    // test instruction sub
    emu.run(Some(0x40102a));
    assert!(!emu.flags().f_cf);
    assert!(!emu.flags().f_of);
    assert!(emu.flags().f_zf);
    assert!(!emu.flags().f_sf);
    assert!(emu.flags().f_pf);

    // test instruction cmp
    emu.run(Some(0x401040));
    assert!(emu.flags().f_cf);
    assert!(!emu.flags().f_of);
    assert!(!emu.flags().f_zf);
    assert!(emu.flags().f_sf);
    assert!(!emu.flags().f_pf);

    // test instruction test
    emu.run(Some(0x401056));
    assert!(!emu.flags().f_cf);
    assert!(!emu.flags().f_of);
    assert!(emu.flags().f_zf);
    assert!(!emu.flags().f_sf);
    assert!(emu.flags().f_pf);

    // test and
    emu.run(Some(0x40106c));
    assert!(!emu.flags().f_cf);
    assert!(!emu.flags().f_of);
    assert!(emu.flags().f_zf);
    assert!(!emu.flags().f_sf);
    assert!(emu.flags().f_pf);

    // test or with 0x0
    emu.run(Some(0x401087));
    assert!(!emu.flags().f_cf);
    assert!(!emu.flags().f_of);
    assert!(!emu.flags().f_zf);
    assert!(emu.flags().f_sf);
    assert!(emu.flags().f_pf);

    // test shl
    emu.run(Some(0x40109d));
    assert!(emu.flags().f_cf);
    assert!(emu.flags().f_of);
    assert!(emu.flags().f_zf);
    assert!(!emu.flags().f_sf);
    assert!(emu.flags().f_pf);

    // test add
    emu.run(Some(0x4010b8));
    assert!(!emu.flags().f_cf);
    assert!(emu.flags().f_of);
    assert!(!emu.flags().f_zf);
    assert!(emu.flags().f_sf);
    assert!(emu.flags().f_pf);
}
