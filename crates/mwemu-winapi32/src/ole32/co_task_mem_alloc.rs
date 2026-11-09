use crate::emu::Emu;
use crate::winapi::common::crt;

pub(super) fn CoTaskMemAlloc(emu: &mut Emu) {
    crt::task_alloc(emu);
}
