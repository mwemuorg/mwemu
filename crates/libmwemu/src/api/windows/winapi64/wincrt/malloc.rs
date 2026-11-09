use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn malloc(emu: &mut Emu) {
    crt::malloc(emu);
}
