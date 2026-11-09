use crate::emu;
use crate::winapi::helper;

pub fn CreateBrushIndirect(emu: &mut emu::Emu) {
    let lbrush = emu.regs().rcx;
    let (style, color) = if lbrush != 0 {
        (
            emu.maps.read_dword(lbrush).unwrap_or(0),
            emu.maps.read_dword(lbrush + 4).unwrap_or(0),
        )
    } else {
        (0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreateBrushIndirect lbrush:0x{:x} style:{} color:0x{:06x}",
        lbrush,
        style,
        color
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://brush");
}
