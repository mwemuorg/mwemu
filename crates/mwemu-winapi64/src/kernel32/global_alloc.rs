use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn GlobalAlloc(emu: &mut Emu) {
    heap::local_alloc_api(emu);
}
