use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn crt_memmove(emu: &mut Emu) {
    crt::copy(emu);
}
