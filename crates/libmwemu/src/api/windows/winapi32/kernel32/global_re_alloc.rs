use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn GlobalReAlloc(emu: &mut Emu) {
    heap::local_realloc_api(emu);
}
