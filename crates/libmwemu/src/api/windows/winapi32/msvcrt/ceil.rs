use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn ceil(emu: &mut Emu) {
    crt::math_unary(emu, "ceil", f64::ceil);
}
