use crate::emu::Emu;
use crate::winapi::common::crt::{self, Runtime::Msvcrt};

pub fn _ftol(emu: &mut Emu) {
    crt::float_to_integer(emu, "_ftol", Msvcrt);
}
