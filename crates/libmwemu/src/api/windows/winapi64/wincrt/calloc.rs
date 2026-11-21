use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn calloc(emu: &mut Emu) {
    crt::calloc(emu);
}
