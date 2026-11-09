use crate::emu::Emu;
use crate::winapi::common::memory::flush_instruction_cache;

pub fn FlushInstructionCache(emu: &mut Emu) {
    flush_instruction_cache(emu, "kernelbase!FlushInstructionCache");
}
