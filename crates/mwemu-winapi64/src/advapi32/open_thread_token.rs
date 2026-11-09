use crate::emu;
use crate::winapi::helper;

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let _open_as_self = emu.regs().r8 != 0;
    let token_handle_ptr = emu.regs().r9;
    let token = helper::handler_create("token");
    if token_handle_ptr != 0 {
        emu.maps.write_qword(token_handle_ptr, token);
    }
    log_red!(
        emu,
        "advapi32!OpenThreadToken thread:0x{:x} access:0x{:x} => 0x{:x}",
        thread_handle,
        desired_access,
        token
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
