use crate::emu;

pub fn MoveToEx(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let lp_point = emu.regs().r9;
    // If lpPoint is non-null, write the previous position (0,0)
    if lp_point != 0 {
        emu.maps.write_dword(lp_point, 0); // previous x
        emu.maps.write_dword(lp_point + 4, 0); // previous y
    }
    log_red!(
        emu,
        "gdi32!MoveToEx hdc:0x{:x} x:{} y:{} lpPoint:0x{:x}",
        hdc,
        x,
        y,
        lp_point
    );
    emu.regs_mut().rax = 1;
}
