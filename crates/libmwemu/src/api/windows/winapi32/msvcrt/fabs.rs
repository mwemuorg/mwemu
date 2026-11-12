use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn fabs(emu: &mut Emu) {
    crt::math_unary(emu, "fabs", f64::abs);
}
