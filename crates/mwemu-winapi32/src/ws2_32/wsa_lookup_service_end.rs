use crate::emu;
use crate::winapi::helper;

pub fn WSALookupServiceEnd(emu: &mut emu::Emu) {
    let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(
        emu,
        "ws2_32!WSALookupServiceEnd handle:0x{:x}",
        lookup_handle
    );
    if helper::handler_exist(lookup_handle) {
        helper::handler_close(lookup_handle);
        emu.regs_mut().rax = 0;
    } else {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0xFFFFFFFF;
    }
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
