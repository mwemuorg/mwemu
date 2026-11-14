use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn memcpy(emu: &mut Emu) {
    crt::copy(emu);
}
