use crate::emu;
use crate::winapi::helper;

pub fn WSAInstallServiceClassA(emu: &mut emu::Emu) {
    let class_info_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!WSAInstallServiceClassA ptr:0x{:x}",
        class_info_ptr
    );
    let handle = helper::handler_create("WSAInstallServiceClassA");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
