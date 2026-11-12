use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CallWindowProcA(emu: &mut emu::Emu) {
    call_window_proc(emu, "CallWindowProcA", false);
}

fn call_window_proc(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = WinApiAbi::arg(emu, 4);
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

pub fn CallWindowProcW(emu: &mut emu::Emu) {
    call_window_proc(emu, "CallWindowProcW", true);
}
