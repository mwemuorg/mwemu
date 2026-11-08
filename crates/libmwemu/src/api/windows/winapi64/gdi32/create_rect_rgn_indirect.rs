use crate::emu;
use crate::winapi::helper;

pub fn CreateRectRgnIndirect(emu: &mut emu::Emu) {
    let rect = emu.regs().rcx;
    let (left, top, right, bottom) = if rect != 0 {
        (
            emu.maps.read_dword(rect).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 4).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 8).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 12).unwrap_or(0) as i32,
        )
    } else {
        (0, 0, 0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreateRectRgnIndirect rect:({},{},{},{})",
        left,
        top,
        right,
        bottom
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://region");
}
