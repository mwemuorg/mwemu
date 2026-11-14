use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn realloc(emu: &mut Emu) {
    crt::realloc(emu);
}
