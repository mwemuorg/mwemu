use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn cos(emu: &mut Emu) {
    crt::math_unary(emu, "cos", f64::cos);
}
