use crate::emu;
use crate::winapi::helper;

pub fn CreatePenIndirect(emu: &mut emu::Emu) {
    let lpen = emu.regs().rcx;
    let (style, width, color) = if lpen != 0 {
        (
            emu.maps.read_dword(lpen).unwrap_or(0),
            emu.maps.read_dword(lpen + 4).unwrap_or(0) as i32,
            emu.maps.read_dword(lpen + 12).unwrap_or(0),
        )
    } else {
        (0, 0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreatePenIndirect lpen:0x{:x} style:{} width:{} color:0x{:06x}",
        lpen,
        style,
        width,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://pen");
}
