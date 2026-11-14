use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    let thread_handle = WinApiAbi::arg(emu, 0) as u32;
    let desired_access = WinApiAbi::arg(emu, 1) as u32;
    let open_as_self = WinApiAbi::arg(emu, 2) as u32 != 0;
    let token_handle_ptr = WinApiAbi::arg(emu, 3);
    let token = helper::handler_create("token");
    if token_handle_ptr != 0 {
        emu.maps.write_dword(token_handle_ptr, token as u32);
    }
    log_red!(
        emu,
        "advapi32!OpenThreadToken thread:0x{:x} access:0x{:x} => 0x{:x}",
        thread_handle,
        desired_access,
        token
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
