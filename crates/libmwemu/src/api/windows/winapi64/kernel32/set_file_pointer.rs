use crate::emu;

pub fn SetFilePointer(emu: &mut emu::Emu) {
    let hfile = emu.regs().rcx;
    let dist_low = emu.regs().rdx as i32;
    let dist_high_ptr = emu.regs().r8;
    let method = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!SetFilePointer hFile:0x{:x} dist:{} high_ptr:0x{:x} method:{}",
        hfile,
        dist_low,
        dist_high_ptr,
        method
    );

    emu.regs_mut().rax = dist_low as u64;
}
