use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn HeapCreate(emu: &mut Emu) {
    heap::create_api(emu);
}
