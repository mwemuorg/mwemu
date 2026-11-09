use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn HeapWalk(emu: &mut emu::Emu) {
    let heap = arg(emu, 0) as u64;
    let entry = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!HeapWalk heap:0x{:x} entry:0x{:x}",
        heap,
        entry
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // FALSE (no more entries)
    stdcall_ret(emu, 2);
}
