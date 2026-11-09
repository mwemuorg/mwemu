use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn crt_aligned_malloc(emu: &mut Emu) {
    crt::aligned_malloc(emu);
}
