use crate::emu;
use crate::winapi::helper;

pub fn CreateFontIndirectW(emu: &mut emu::Emu) {
    let lf = emu.regs().rcx;
    let height = if lf != 0 {
        emu.maps.read_dword(lf).unwrap_or(0) as i32
    } else {
        0
    };
    log_red!(
        emu,
        "gdi32!CreateFontIndirectW lf:0x{:x} height:{}",
        lf,
        height
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://font");
}
