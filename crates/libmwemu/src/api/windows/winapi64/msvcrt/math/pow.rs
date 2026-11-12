use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn pow(emu: &mut Emu) {
    crt::math_binary(emu, "pow", f64::powf);
}
