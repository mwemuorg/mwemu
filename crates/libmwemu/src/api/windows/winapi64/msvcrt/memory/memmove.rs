use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn memmove(emu: &mut Emu) {
    crt::copy(emu);
}
