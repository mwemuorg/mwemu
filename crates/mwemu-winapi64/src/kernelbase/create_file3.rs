use crate::emu;
use crate::winapi::helper;

pub fn CreateFile3(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateFile3");
    log_red!(emu, "kernelbase!CreateFile3 => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
