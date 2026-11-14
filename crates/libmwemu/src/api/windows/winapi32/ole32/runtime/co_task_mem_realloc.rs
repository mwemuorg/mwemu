use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn CoTaskMemRealloc(emu: &mut Emu) {
    crt::task_realloc(emu);
}
