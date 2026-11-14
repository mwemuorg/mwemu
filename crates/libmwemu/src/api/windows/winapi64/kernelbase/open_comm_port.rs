use crate::emu;
use crate::winapi::helper;

pub fn OpenCommPort(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("OpenCommPort");
    log_red!(emu, "kernelbase!OpenCommPort => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
