use crate::emu;

pub fn SetWindowOrgEx(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let lp_point = emu.regs().r9;
    // Write previous origin to lpPoint if non-null
    if lp_point != 0 {
        emu.maps.write_dword(lp_point, 0); // previous x
        emu.maps.write_dword(lp_point + 4, 0); // previous y
    }
    log_red!(
        emu,
        "gdi32!SetWindowOrgEx hdc:0x{:x} x:{} y:{} lpPoint:0x{:x}",
        hdc,
        x,
        y,
        lp_point
    );
    emu.regs_mut().rax = 1;
}
