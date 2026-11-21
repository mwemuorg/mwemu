use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn HeapSize(emu: &mut Emu) {
    heap::size_api(emu);
}
