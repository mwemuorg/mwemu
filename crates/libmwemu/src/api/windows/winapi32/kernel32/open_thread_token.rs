use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    let thread_handle = arg(emu, 0) as u64;
    let desired_access = arg(emu, 1) as u64 as u32;
    let open_as_self = arg(emu, 2) as u64 as u32;
    let token_handle_ptr = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!OpenThreadToken thread: 0x{:x} access: 0x{:x} as_self: {}",
        thread_handle,
        desired_access,
        open_as_self
    );

    if token_handle_ptr != 0 {
        let token = helper::handler_create("token");
        emu.maps.write_qword(token_handle_ptr, token);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
