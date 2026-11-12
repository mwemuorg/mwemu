use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ExtTextOutA(emu: &mut emu::Emu) {
    ext_text_out(emu, "ExtTextOutA", false);
}

fn ext_text_out(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let options = emu.regs().r9 as u32;
    let _lprect = WinApiAbi::arg(emu, 4);
    let text_ptr = WinApiAbi::arg(emu, 5);
    let count = WinApiAbi::arg(emu, 6) as u32;
    let text = if text_ptr != 0 && count > 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} x:{} y:{} options:0x{:x} count:{} text:'{}'",
        hdc,
        x,
        y,
        options,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}

pub fn ExtTextOutW(emu: &mut emu::Emu) {
    ext_text_out(emu, "ExtTextOutW", true);
}
