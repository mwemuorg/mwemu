use crate::emu::Emu;
use crate::winapi::common::memory_ops::{self, Operation};

pub fn RtlMoveMemory(emu: &mut Emu) {
    memory_ops::run(emu, Operation::Copy);
}
