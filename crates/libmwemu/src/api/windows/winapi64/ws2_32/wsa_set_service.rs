use crate::emu;
use crate::winapi::common::strings;

pub fn WSASetServiceA(emu: &mut emu::Emu) {
    wsa_set_service(emu, "WSASetServiceA", false);
}

fn wsa_set_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let query_set_ptr = emu.regs().rcx;
    let operation = emu.regs().rdx;
    let flags = emu.regs().r8;
    log_red!(emu, "ws2_32!{api} op:{} flags:0x{:x}", operation, flags);
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn WSASetServiceW(emu: &mut emu::Emu) {
    wsa_set_service(emu, "WSASetServiceW", true);
}
