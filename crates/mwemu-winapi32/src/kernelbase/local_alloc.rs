use crate::emu;
use crate::winapi::winapi32::kernel32;

pub(super) fn LocalAlloc(emu: &mut emu::Emu) {
    // LocalAlloc(uFlags, uBytes) is equivalent to HeapAlloc(GetProcessHeap(), uFlags, uBytes)
    // push a dummy handle for the global heap
    emu.stack_push32(0x0);
    kernel32::HeapAlloc(emu);
}
