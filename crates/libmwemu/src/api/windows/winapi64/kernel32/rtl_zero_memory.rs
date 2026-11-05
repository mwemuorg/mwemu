use crate::emu;

pub fn RtlZeroMemory(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let len = emu.regs().rdx as usize;

    log_red!(emu, "kernel32!RtlZeroMemory dest:0x{:x} len:{}", dest, len);

    if dest != 0 && len > 0 && len <= 0x100_0000 {
        emu.maps.memset(dest, 0, len);
    }
}
