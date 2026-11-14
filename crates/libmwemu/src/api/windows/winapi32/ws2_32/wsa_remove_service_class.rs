use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSARemoveServiceClass(emu: &mut emu::Emu) {
    let class_id_ptr = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WSARemoveServiceClass ptr:0x{:x}", class_id_ptr);
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
