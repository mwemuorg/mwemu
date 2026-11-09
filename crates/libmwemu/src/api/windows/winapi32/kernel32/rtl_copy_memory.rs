use crate::emu::Emu;
use crate::winapi::common::memory_ops::{self, Operation};

pub fn RtlCopyMemory(emu: &mut Emu) {
    memory_ops::run(emu, Operation::Copy);
}
