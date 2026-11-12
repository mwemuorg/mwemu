use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAGetServiceClassInfoA(emu: &mut emu::Emu) {
    wsa_get_service_class_info(emu, "WSAGetServiceClassInfoA");
}

fn wsa_get_service_class_info(emu: &mut emu::Emu, api: &str) {
    let provider_id = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let class_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
    stdcall_ret(emu, 4);
}

pub fn WSAGetServiceClassInfoW(emu: &mut emu::Emu) {
    wsa_get_service_class_info(emu, "WSAGetServiceClassInfoW");
}
