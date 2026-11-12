use crate::emu;
use crate::winapi::helper;

pub fn CreateFontIndirectA(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectA");
}

fn create_font_indirect(emu: &mut emu::Emu, api: &str) {
    let lf = emu.regs().rcx;
    let height = if lf != 0 {
        emu.maps.read_dword(lf).unwrap_or(0) as i32
    } else {
        0
    };
    log_red!(emu, "gdi32!{api} lf:0x{:x} height:{}", lf, height);
    emu.regs_mut().rax = helper::handler_create("gdi32://font");
}

pub fn CreateFontIndirectW(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectW");
}
