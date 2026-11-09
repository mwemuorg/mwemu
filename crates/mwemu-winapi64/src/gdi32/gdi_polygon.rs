use crate::emu;
use crate::winapi::helper;

pub fn GdiPolygon(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let points = emu.regs().rdx;
    let count = emu.regs().r8;
    log_red!(
        emu,
        "gdi32!Polygon hdc:0x{:x} pts:0x{:x} count:{}",
        hdc,
        points,
        count
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
