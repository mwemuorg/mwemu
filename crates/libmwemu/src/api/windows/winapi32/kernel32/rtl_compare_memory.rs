use crate::emu::Emu;
use crate::winapi::common::memory_ops;

pub fn RtlCompareMemory(emu: &mut Emu) {
    memory_ops::dispatch("RtlCompareMemory", emu);
}
