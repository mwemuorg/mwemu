use crate::engine::logic;
use crate::tests::helpers;
use crate::*;

#[test]
// logic tests
pub fn logic_test() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let num: u64 = 0x1234_5678_9ABC_DEF0;
    let shift: u64 = 12;
    let size: u32 = 32;
    let src: u64 = num >> (size as u64 - shift);

    let mut r: u64;
    (r, _) = logic::shrd(&mut emu, 0x9fd88893, 0x1b, 0x6, 32);
    assert!(r == 0x6e7f6222);
    (r, _) = logic::shrd(&mut emu, 0x6fdcb03, 0x0, 0x6, 32);
    assert!(r == 0x1bf72c);
    (r, _) = logic::shrd(&mut emu, 0x91545f1d, 0x6fe2, 0x6, 32);
    assert!(r == 0x8a45517c);
    (r, _) = logic::shld(&mut emu, 0x1b, 0xf1a7eb1d, 0xa, 32);
    assert!(r == 0x6fc6);
    (r, _) = logic::shld(&mut emu, 0x1, 0xffffffff, 4, 32);
    assert!(r == 0x1f);
    (r, _) = logic::shld(&mut emu, 0x1, 0xffffffff, 33, 32);
    assert!(r == 0x3);
    (r, _) = logic::shld(&mut emu, 0x144e471f8, 0x14F498, 0x3e, 64);
    assert!(r == 0x53d26);
}
