use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn free(emu: &mut Emu) {
    crt::free(emu);
}
