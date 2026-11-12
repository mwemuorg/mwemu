use crate::emu;

pub(super) fn SendMessageA(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageA", false);
}

pub(super) fn SendMessageW(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageW", true);
}

fn send_message(emu: &mut emu::Emu, api: &str, wide: bool) {
    log_red!(
        emu,
        "user32!{api} hwnd:0x{:x} msg:0x{:x} wParam:0x{:x} lParam:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx,
        emu.regs().r8,
        emu.regs().r9
    );
    if wide {
    } else {
        emu.set_last_error(0);
    }

    emu.regs_mut().rax = 0;
}
