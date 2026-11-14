use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn TabbedTextOutA(emu: &mut emu::Emu) {
    tabbed_text_out(emu, "TabbedTextOutA", false);
}

fn tabbed_text_out(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = WinApiAbi::arg(emu, 4);
    let p5 = WinApiAbi::arg(emu, 5);
    let p6 = WinApiAbi::arg(emu, 6);
    let p7 = WinApiAbi::arg(emu, 7);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "user32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
}

pub fn TabbedTextOutW(emu: &mut emu::Emu) {
    tabbed_text_out(emu, "TabbedTextOutW", true);
}
