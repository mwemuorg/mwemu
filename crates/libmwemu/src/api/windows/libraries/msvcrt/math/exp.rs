use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn exp(emu: &mut Emu) {
    crt::math_unary(emu, "exp", f64::exp);
}
