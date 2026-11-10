use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn memset(emu: &mut Emu) {
    crt::fill(emu);
}
