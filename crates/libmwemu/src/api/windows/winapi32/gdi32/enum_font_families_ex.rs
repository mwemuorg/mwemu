use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn EnumFontFamiliesExA(emu: &mut emu::Emu) {
    enum_font_families_ex(emu, "EnumFontFamiliesExA");
}

fn enum_font_families_ex(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    let arg3 = WinApiAbi::arg(emu, 2) as u32;
    let arg4 = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "gdi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EnumFontFamiliesExW(emu: &mut emu::Emu) {
    enum_font_families_ex(emu, "EnumFontFamiliesExW");
}
