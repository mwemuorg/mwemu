use crate::emu;

pub fn ExcludeClipRect(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!ExcludeClipRect hdc:0x{:x} l:{} t:{} r:{} b:{}",
        hdc,
        left,
        top,
        right,
        bottom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}
