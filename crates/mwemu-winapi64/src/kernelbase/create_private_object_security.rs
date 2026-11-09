use crate::emu;
use crate::winapi::helper;

pub fn CreatePrivateObjectSecurity(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreatePrivateObjectSecurity");
    log_red!(
        emu,
        "kernelbase!CreatePrivateObjectSecurity => 0x{:x}",
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
