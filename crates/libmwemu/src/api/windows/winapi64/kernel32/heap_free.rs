use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn HeapFree(emu: &mut Emu) {
    heap::free_api(emu);
}
