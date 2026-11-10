use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn crt_msize(emu: &mut Emu) {
    crt::msize(emu);
}
