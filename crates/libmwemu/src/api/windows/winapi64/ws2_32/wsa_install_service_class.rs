use crate::emu;
use crate::winapi::helper;

pub fn WSAInstallServiceClassA(emu: &mut emu::Emu) {
    wsa_install_service_class(emu, "WSAInstallServiceClassA");
}

fn wsa_install_service_class(emu: &mut emu::Emu, api: &str) {
    let class_info_ptr = emu.regs().rcx;
    log_red!(emu, "ws2_32!{api} ptr:0x{:x}", class_info_ptr);
    let handle = helper::handler_create(api);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn WSAInstallServiceClassW(emu: &mut emu::Emu) {
    wsa_install_service_class(emu, "WSAInstallServiceClassW");
}
