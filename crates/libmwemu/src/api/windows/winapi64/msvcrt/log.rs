use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn log(emu: &mut Emu) {
    crt::math_unary(emu, "log", f64::ln);
}
