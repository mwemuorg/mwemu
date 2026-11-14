use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let process_handle = WinApiAbi::arg(emu, 0) as u32;
    let desired_access = WinApiAbi::arg(emu, 1) as u32;
    let token_handle_ptr = WinApiAbi::arg(emu, 2);
    let token = helper::handler_create("token");
    if token_handle_ptr != 0 {
        emu.maps.write_dword(token_handle_ptr, token as u32);
    }
    log_red!(
        emu,
        "advapi32!OpenProcessToken proc:0x{:x} access:0x{:x} => 0x{:x}",
        process_handle,
        desired_access,
        token
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
