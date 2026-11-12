use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn calloc(emu: &mut Emu) {
    crt::calloc(emu);
}
