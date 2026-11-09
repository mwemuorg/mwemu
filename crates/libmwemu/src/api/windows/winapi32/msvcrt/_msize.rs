use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn _msize(emu: &mut Emu) {
    crt::msize(emu);
}
