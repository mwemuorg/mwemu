use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn HeapDestroy(emu: &mut Emu) {
    heap::destroy_api(emu, false);
}
