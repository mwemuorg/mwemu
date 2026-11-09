use crate::emu;

pub fn GetSysColor(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;
    let _ = emu.maps.is_mapped(index);
    log_red!(emu, "user32!GetSysColor index:{}", index);
    let color: u64 = match index {
        0 => 0xd4d0c8,  // COLOR_SCROLLBAR
        1 => 0x004e98,  // COLOR_BACKGROUND
        2 => 0x0a246a,  // COLOR_ACTIVECAPTION
        4 => 0xd4d0c8,  // COLOR_MENU
        5 => 0xffffff,  // COLOR_WINDOW
        8 => 0x000000,  // COLOR_WINDOWTEXT
        15 => 0xd4d0c8, // COLOR_3DFACE
        _ => 0xd4d0c8,
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = color;
}
