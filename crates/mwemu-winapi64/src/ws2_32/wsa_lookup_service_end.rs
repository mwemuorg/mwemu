use crate::emu;
use crate::winapi::helper;

pub fn WSALookupServiceEnd(emu: &mut emu::Emu) {
    let lookup_handle = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!WSALookupServiceEnd handle:0x{:x}",
        lookup_handle
    );
    if helper::handler_exist(lookup_handle) {
        helper::handler_close(lookup_handle);
        emu.regs_mut().rax = 0;
    } else {
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0xFFFFFFFF;
    }
}
