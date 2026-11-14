use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub(super) fn MessageBoxA(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxA", false);
}

fn message_box(emu: &mut emu::Emu, api: &str, wide: bool) {
    let titleptr = WinApiAbi::arg(emu, 2);
    let msgptr = WinApiAbi::arg(emu, 1);
    let msg = strings::read_text(emu, msgptr, wide);
    let title = strings::read_text(emu, titleptr, wide);

    log_red!(emu, "user32!{api} {} {}", title, msg);

    emu.regs_mut().rax = 0;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub(super) fn MessageBoxW(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxW", true);
}
