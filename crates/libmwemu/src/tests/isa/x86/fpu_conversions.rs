use crate::fpu::FPU;
use crate::tests::helpers;

#[test]
#[allow(clippy::approx_constant)]
// basic tests of some fpu functionst.
pub fn fpu_conversions() {
    helpers::setup();

    let mut fpu = FPU::new();
    assert_eq!(fpu.get_top(), 0);
    assert_eq!(fpu.get_depth(), 0);

    fpu.push_f64(0.0);
    fpu.push_f64(1.0);
    assert_eq!(fpu.peek_st_logical_f64(0), 1.0);
    assert_eq!(fpu.peek_st_logical_f64(1), 0.0);

    // u80 to f64 conversion
    fpu.set_st_u80(1, 0x4000c90fdaa22168c235);
    fpu.st.print();
    assert_eq!(fpu.peek_st_logical_f64(1), 3.14159265358979323);
    assert_eq!(fpu.peek_st_logical_u80(1), 0x4000c90fdaa22168c235);

    /*
    assert_eq!(3.141592653589793239,
                3.141592653589793);  // true cuts to 64bits
                                    //
    */

    // f64 to u80 conversion
    //fpu.set_st(1, 4.141592653589793238);
    //assert_eq!(fpu.peek_st_u80(1), 0x4000c90fdaa22168c234);
    //
}
