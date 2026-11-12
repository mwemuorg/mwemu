use crate::emu;
use crate::winapi::common::strings;

pub fn WSASetServiceA(emu: &mut emu::Emu) {
    wsa_set_service(emu, "WSASetServiceA", false);
}

fn wsa_set_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let query_set_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(emu, "ws2_32!{api} op:{} flags:0x{:x}", operation, flags);
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSASetServiceW(emu: &mut emu::Emu) {
    wsa_set_service(emu, "WSASetServiceW", true);
}
