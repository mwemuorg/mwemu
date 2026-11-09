use crate::emu;
use crate::winapi::helper;

pub fn GetStockObject(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;
    let name = match index {
        0 => "WHITE_BRUSH",
        1 => "LTGRAY_BRUSH",
        2 => "GRAY_BRUSH",
        3 => "DKGRAY_BRUSH",
        4 => "BLACK_BRUSH",
        5 => "NULL_BRUSH",
        6 => "WHITE_PEN",
        7 => "BLACK_PEN",
        8 => "NULL_PEN",
        10 => "OEM_FIXED_FONT",
        11 => "ANSI_FIXED_FONT",
        12 => "ANSI_VAR_FONT",
        13 => "SYSTEM_FONT",
        14 => "DEVICE_DEFAULT_FONT",
        15 => "DEFAULT_PALETTE",
        16 => "SYSTEM_FIXED_FONT",
        17 => "DEFAULT_GUI_FONT",
        18 => "DC_BRUSH",
        19 => "DC_PEN",
        _ => "UNKNOWN",
    };
    log_red!(emu, "gdi32!GetStockObject index:{} ({})", index, name);
    emu.regs_mut().rax = helper::handler_create("gdi32://stock_object");
}
