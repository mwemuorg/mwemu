use crate::emu;
use crate::winapi::helper;

pub fn SetEndOfFile(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernelbase!SetEndOfFile");
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
