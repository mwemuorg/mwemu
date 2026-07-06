use crate::tests::helpers;
use crate::*;

#[test]
#[allow(clippy::approx_constant)]
// this tests the fpu unit.
pub fn elf64lin_fpu() {
    helpers::setup();

    let mut emu = emu64();

    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = helpers::test_data_path("elf64lin_fpu.bin");
    emu.load_code(&sample);
    emu.fpu_mut().clear();
    emu.fpu_mut().trace = true;
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0);
    emu.step(); // 1 fninit
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0);
    emu.step(); // 2 fld1
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), 1.0);
    emu.step(); // 3 fldpi
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x4000c90fdaa22168c234); // should end in 235
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 3.141592653589793);
    emu.step(); // 4 fadd   st,st(1)
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x40018487ed5110b4611a);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 4.141592653589793);
    emu.step(); // 5 fsub   st,st(1)
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x4000c90fdaa22168c234);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 3.141592653589793);
    emu.step(); // 6 fsubr  st,st(1)
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0xc000890fdaa22168c234);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), -2.141592653589793238);
    emu.step(); // 7 fchs
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x4000890fdaa22168c234);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 2.141592653589793);
    emu.step(); // 8 fsqrt
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fffbb51491ea66b7000); // should end in 6ea4,
    // its comupted as f64
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 1.4634181403788165);

    emu.step(); //  9 fxch   st(1)
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3fffbb51491ea66b7000); // should end in 6ea4
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), 1.4634181403788165);
    emu.step(); //  10 fptan
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3fffbb51491ea66b7000); // should end in 6ea4
    if emu.fpu_mut().peek_st_u80(6) != 0x3fffc75922e5f71d3000 {
        log::info!("f64:tan() -> 0x{:x}", emu.fpu_mut().peek_st_u80(6));
        return; // in mac  f64::tan() returns different value
    }
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fffc75922e5f71d3000);
    assert_eq!(emu.fpu_mut().peek_st_u80(5), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), 1.4634181403788165);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 1.5574077246549023);
    assert_eq!(emu.fpu_mut().peek_st_f64(5), 1.0);
    emu.step(); //  11 fmulp  st(1),st
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3fffbb51491ea66b7000); // should end in 6ea4
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fffc75922e5f71d3000); // should end in 2dc6
    assert_eq!(emu.fpu_mut().peek_st_f64(7), 1.4634181403788165);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 1.5574077246549023);
    emu.step(); // 12 fdivp  st(1),st
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3ffef08ce6b636464000); // should end in 375
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fffc75922e5f71d3000); // should end in 2dc6
    assert_eq!(emu.fpu_mut().peek_st_u80(5), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), 0.9396499819615878);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 1.5574077246549023);
    emu.step(); // 13 fsubp  st(1),st
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
    emu.step(); // 14 f2xm1
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
    emu.step(); // 15 fld1
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0x3fff8000000000000000); // should end in 375
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fffc75922e5f71d3000); // should end in 2dc6
    assert_eq!(emu.fpu_mut().peek_st_u80(5), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
    emu.step(); // 16 fldlg2
    assert_eq!(emu.fpu_mut().st.get_top(), 6);
    assert_eq!(emu.fpu_mut().st.get_depth(), 2);
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3ffd9a209a84fbcff800); // 799);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), 0.3010299956639812);
    emu.step(); // 17 fyl2x
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), -1.7320208456446193);
    emu.step(); // 18 fld1
    emu.step(); // 19 fld1
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_u80(5), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), -1.7320208456446193);
    emu.step(); // 20 fyl2xp1
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_u80(5), 0x3fff8000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), -1.7320208456446193);
    emu.step(); // 21 fucom  st(1)
    emu.step(); // 22 fcmovnbe st(0), st(1)
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0xbfffddb2dbec0456f800); //46);
    assert_eq!(emu.fpu_mut().peek_st_f64(7), -1.7320208456446193);
    assert_eq!(emu.fpu_mut().peek_st_f64(6), -1.7320208456446193);
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
    emu.step(); // 23 fcmovnu st(0), st(1)
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
    emu.step(); // fstp   st(0)
    emu.step(); // fstp   st(0)
    emu.step(); // fstp   st(0)
    assert_eq!(emu.fpu_mut().peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
    assert_eq!(emu.fpu_mut().peek_st_u80(6), 0xbfffddb2dbec0456f800); //46);
    assert_eq!(emu.fpu_mut().peek_st_u80(0), 0xffffc000000000000000);
}
