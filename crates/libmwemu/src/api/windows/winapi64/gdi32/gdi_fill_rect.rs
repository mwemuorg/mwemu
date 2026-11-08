use crate::emu;

pub fn GdiFillRect(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let rect = emu.regs().rdx;
    let brush = emu.regs().r8;
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
        "gdi32!FillRect hdc:0x{:x} rect:({},{},{},{}) brush:0x{:x}",
        hdc,
        left,
        top,
        right,
        bottom,
        brush
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
