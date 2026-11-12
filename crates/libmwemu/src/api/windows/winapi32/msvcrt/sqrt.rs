use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn sqrt(emu: &mut Emu) {
    crt::math_unary(emu, "sqrt", f64::sqrt);
}
