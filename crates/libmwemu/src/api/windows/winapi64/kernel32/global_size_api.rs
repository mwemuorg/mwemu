use crate::emu::Emu;
use crate::winapi::common::heap;

pub fn GlobalSize_api(emu: &mut Emu) {
    heap::local_size_api(emu);
}
