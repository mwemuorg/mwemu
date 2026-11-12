use crate::emu;

pub(super) fn DispatchMessageA(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageA", false);
}

pub(super) fn DispatchMessageW(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageW", true);
}

fn dispatch_message(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!{api} arg1:0x{:x}", arg1);
    if wide {
    } else {
        emu.set_last_error(0);
    }

    emu.regs_mut().rax = 0;
}
