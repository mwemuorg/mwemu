use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn _ftol2(emu: &mut Emu, api: &str) {
    crt::float_to_integer(emu, api);
}
