use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn free(emu: &mut Emu) {
    crt::free(emu);
}
