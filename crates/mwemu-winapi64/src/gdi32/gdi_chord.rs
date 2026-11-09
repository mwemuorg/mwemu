use crate::emu;

pub fn GdiChord(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let left = emu.regs().rdx as i32;
    let top = emu.regs().r8 as i32;
    let right = emu.regs().r9 as i32;
    let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let x_start = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let y_start = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as i32;
    let x_end = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as i32;
    let y_end = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as i32;
    log_red!(
        emu,
        "gdi32!Chord hdc:0x{:x} rect:({},{},{},{}) start:({},{}) end:({},{})",
        hdc,
        left,
        top,
        right,
        bottom,
        x_start,
        y_start,
        x_end,
        y_end
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
