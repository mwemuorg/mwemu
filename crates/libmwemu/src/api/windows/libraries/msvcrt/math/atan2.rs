use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn atan2(emu: &mut Emu) {
    crt::math_binary(emu, "atan2", f64::atan2);
}
