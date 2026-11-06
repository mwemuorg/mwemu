
use crate::emu;
use crate::winapi::helper;
pub fn DeleteProcThreadAttributeList(emu: &mut emu::Emu) {
    let attr_list = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!DeleteProcThreadAttributeList list:0x{:x}",
        attr_list
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
}
