use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn TextOutA(emu: &mut emu::Emu) {
    text_out(emu, "TextOutA", false);
}

fn text_out(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let text_ptr = emu.regs().r9;
    let count = WinApiAbi::arg(emu, 4) as i32;
    let text = if text_ptr != 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} x:{} y:{} count:{} text:'{}'",
        hdc,
        x,
        y,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}

pub fn TextOutW(emu: &mut emu::Emu) {
    text_out(emu, "TextOutW", true);
}
