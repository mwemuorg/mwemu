use crate::emu;
use crate::winapi::helper;

pub fn WSACancelAsyncRequest(emu: &mut emu::Emu) {
    let async_handle = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!WSACancelAsyncRequest handle:0x{:x}",
        async_handle
    );
    if helper::handler_exist(async_handle) {
        helper::handler_close(async_handle);
        emu.regs_mut().rax = 0;
    } else {
        emu.set_last_error(10044); // WSAEINVAL
        emu.regs_mut().rax = 0xFFFFFFFF;
    }
}
