use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn memcpy(emu: &mut Emu) {
    crt::copy(emu);
}
