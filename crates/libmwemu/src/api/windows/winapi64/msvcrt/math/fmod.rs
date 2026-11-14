use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn fmod(emu: &mut Emu) {
    crt::math_binary(emu, "fmod", |left, right| left % right);
}
