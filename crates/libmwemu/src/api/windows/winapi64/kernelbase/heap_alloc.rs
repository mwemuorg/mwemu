use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn HeapAlloc(emu: &mut Emu) {
    heap::alloc_api(emu);
}
