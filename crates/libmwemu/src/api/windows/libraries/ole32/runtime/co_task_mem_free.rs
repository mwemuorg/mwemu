use crate::emu::Emu;
use crate::winapi::common::crt;

pub fn CoTaskMemFree(emu: &mut Emu) {
    crt::task_free(emu);
}
