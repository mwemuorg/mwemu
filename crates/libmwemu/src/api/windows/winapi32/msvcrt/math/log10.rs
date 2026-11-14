use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn log10(emu: &mut Emu) {
    crt::math_unary(emu, "log10", f64::log10);
}
