use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn tan(emu: &mut Emu) {
    crt::math_unary(emu, "tan", f64::tan);
}
