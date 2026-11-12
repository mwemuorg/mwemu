use crate::emu;

pub fn WSAGetServiceClassInfoA(emu: &mut emu::Emu) {
    wsa_get_service_class_info(emu, "WSAGetServiceClassInfoA");
}

fn wsa_get_service_class_info(emu: &mut emu::Emu, api: &str) {
    let provider_id = emu.regs().rcx;
    let class_id = emu.regs().rdx;
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
}

pub fn WSAGetServiceClassInfoW(emu: &mut emu::Emu) {
    wsa_get_service_class_info(emu, "WSAGetServiceClassInfoW");
}
