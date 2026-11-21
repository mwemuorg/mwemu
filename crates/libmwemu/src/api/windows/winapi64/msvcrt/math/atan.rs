use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn atan(emu: &mut Emu) {
    crt::math_unary(emu, "atan", f64::atan);
}
