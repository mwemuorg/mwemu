use crate::emu;
use crate::winapi::helper;

pub fn DeleteUmsCompletionList(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(
        emu,
        "kernel32!DeleteUmsCompletionList handle:0x{:x}",
        handle
    );
    emu.regs_mut().rax = 1;
}
