use crate::emu;
use crate::winapi::winapi64::kernel32::HeapAlloc;

pub fn LocalAlloc(emu: &mut emu::Emu) {
    // LocalAlloc(uFlags, uBytes) is equivalent to HeapAlloc(GetProcessHeap(), uFlags, uBytes)
    // rcx = uFlags, rdx = uBytes -> rcx = 0 (global heap), rdx = uFlags, r8 = uBytes
    let flags = emu.regs().rcx;
    let size = emu.regs().rdx;
    emu.regs_mut().rcx = 0;
    emu.regs_mut().rdx = flags;
    emu.regs_mut().r8 = size;
    HeapAlloc(emu);
}
