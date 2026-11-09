use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn malloc(emu: &mut Emu) {
    crt::malloc(emu);
}
