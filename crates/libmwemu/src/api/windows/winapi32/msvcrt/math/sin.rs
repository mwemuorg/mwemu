use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn sin(emu: &mut Emu) {
    crt::math_unary(emu, "sin", f64::sin);
}
