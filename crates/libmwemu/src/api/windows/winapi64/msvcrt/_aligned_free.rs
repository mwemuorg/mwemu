use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn _aligned_free(emu: &mut Emu) {
    crt::aligned_free(emu);
}
