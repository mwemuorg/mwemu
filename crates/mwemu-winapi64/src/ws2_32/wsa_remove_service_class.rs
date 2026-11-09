use crate::emu;
use crate::winapi::helper;

pub fn WSARemoveServiceClass(emu: &mut emu::Emu) {
    let class_id_ptr = emu.regs().rcx;
    log_red!(emu, "ws2_32!WSARemoveServiceClass ptr:0x{:x}", class_id_ptr);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
