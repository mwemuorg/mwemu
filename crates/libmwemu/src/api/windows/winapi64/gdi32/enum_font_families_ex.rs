use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn EnumFontFamiliesExA(emu: &mut emu::Emu) {
    enum_font_families_ex(emu, "EnumFontFamiliesExA");
}

fn enum_font_families_ex(emu: &mut emu::Emu, api: &str) {
    let hdc = emu.regs().rcx;
    let logfont = emu.regs().rdx;
    let proc_ptr = emu.regs().r8;
    let lparam = emu.regs().r9;
    let _flags = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} lf:0x{:x} proc:0x{:x} lparam:0x{:x}",
        hdc,
        logfont,
        proc_ptr,
        lparam
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EnumFontFamiliesExW(emu: &mut emu::Emu) {
    enum_font_families_ex(emu, "EnumFontFamiliesExW");
}
