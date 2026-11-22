use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn HeapReAlloc(emu: &mut Emu) {
    heap::realloc_api(emu);
}
