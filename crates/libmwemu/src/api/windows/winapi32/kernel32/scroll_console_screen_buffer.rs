use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ScrollConsoleScreenBufferA(emu: &mut emu::Emu) {
    scroll_console_screen_buffer(emu, "ScrollConsoleScreenBufferA", false);
}

fn scroll_console_screen_buffer(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let p4 = WinApiAbi::arg(emu, 4);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn ScrollConsoleScreenBufferW(emu: &mut emu::Emu) {
    scroll_console_screen_buffer(emu, "ScrollConsoleScreenBufferW", true);
}
