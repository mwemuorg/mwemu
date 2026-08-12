use crate::emu;
use crate::winapi::winapi32::kernel32::HeapAlloc;

pub fn LocalAlloc(emu: &mut emu::Emu) {
    // We just forward the LocalAlloc to HeapAlloc
    emu.stack_push32(0x0); // push a dummy value consider HeapAlloc for nows always call from the global heap
    HeapAlloc(emu);
}
