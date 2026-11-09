use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn crt_aligned_free(emu: &mut Emu) {
    crt::aligned_free(emu);
}
