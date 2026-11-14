use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ShellMessageBoxA(emu: &mut emu::Emu) {
    shell_message_box(emu, "ShellMessageBoxA", false);
}

fn shell_message_box(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "shell32!{api} '{}'  p1:0x{:x}", s, p1);

    emu.regs_mut().rax = 1;
}

pub fn ShellMessageBoxW(emu: &mut emu::Emu) {
    shell_message_box(emu, "ShellMessageBoxW", true);
}
