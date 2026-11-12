use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAGetServiceClassNameByClassIdA(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdA");
}

fn wsa_get_service_class_name_by_class_id(emu: &mut emu::Emu, api: &str) {
    let class_id_ptr = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
    stdcall_ret(emu, 3);
}

pub fn WSAGetServiceClassNameByClassIdW(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdW");
}
