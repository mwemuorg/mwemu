use crate::emu;
use crate::winapi::helper;

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    let desired_access = emu.regs().rdx as u32;
    let open_as_self = emu.regs().r8 as u32;
    let token_handle_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernelbase!OpenThreadToken thread: 0x{:x} access: 0x{:x} as_self: {}",
        thread_handle,
        desired_access,
        open_as_self
    );
    if token_handle_ptr != 0 {
        let token = helper::handler_create("token");
        emu.maps.write_qword(token_handle_ptr, token);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
