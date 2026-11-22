use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn GlobalFree(emu: &mut Emu) {
    heap::local_free_api(emu);
}
