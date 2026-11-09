use crate::emu;
use crate::winapi::helper;

pub fn GetNextVDMCommand(emu: &mut emu::Emu) {
    let vdm_info_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetNextVDMCommand info:0x{:x} => 0",
        vdm_info_ptr
    );
    let _ = helper::handler_exist(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
