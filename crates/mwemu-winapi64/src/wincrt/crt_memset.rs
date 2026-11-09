use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn crt_memset(emu: &mut Emu) {
    crt::fill(emu);
}
