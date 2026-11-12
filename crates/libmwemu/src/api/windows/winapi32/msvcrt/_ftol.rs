use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn _ftol(emu: &mut Emu) {
    crt::float_to_integer(emu, "_ftol");
}
