use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn WSASetServiceA(emu: &mut emu::Emu) {
    wsa_set_service(emu, "WSASetServiceA", false);
}

fn wsa_set_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let query_set_ptr = WinApiAbi::arg(emu, 0);
    let operation = WinApiAbi::arg(emu, 1) as u32;
    let flags = WinApiAbi::arg(emu, 2) as u32;
    log_red!(emu, "ws2_32!{api} op:{} flags:0x{:x}", operation, flags);
    let val_ptr = WinApiAbi::arg(emu, 1);
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
