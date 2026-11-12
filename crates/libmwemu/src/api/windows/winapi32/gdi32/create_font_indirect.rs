use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateFontIndirectA(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectA");
}

fn create_font_indirect(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "gdi32!{api} arg1:0x{:x}", arg1);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create(&format!("gdi32://{api}"));
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateFontIndirectW(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectW");
}
