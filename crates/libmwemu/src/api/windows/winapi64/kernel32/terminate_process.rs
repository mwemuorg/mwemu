use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;
pub fn TerminateProcess(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let code = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!TerminateProcess hndl: {} code: {}",
        hndl,
        code
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    if hndl == constants::CURRENT_PROCESS_HANDLE {
        emu.stop();
        return;
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
