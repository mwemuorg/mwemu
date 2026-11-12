use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn floor(emu: &mut Emu) {
    crt::math_unary(emu, "floor", f64::floor);
}
