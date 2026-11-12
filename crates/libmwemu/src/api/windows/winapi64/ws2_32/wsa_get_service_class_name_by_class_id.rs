use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSAGetServiceClassNameByClassIdA(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdA");
}

fn wsa_get_service_class_name_by_class_id(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "ws2_32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let out_ptr = WinApiAbi::arg(emu, 2);
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
}

pub fn WSAGetServiceClassNameByClassIdW(emu: &mut emu::Emu) {
    wsa_get_service_class_name_by_class_id(emu, "WSAGetServiceClassNameByClassIdW");
}
