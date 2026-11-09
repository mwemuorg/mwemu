use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn _aligned_malloc(emu: &mut Emu) {
    crt::aligned_malloc(emu);
}
