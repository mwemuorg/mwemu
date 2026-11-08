use crate::emu;
use crate::winapi::helper;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let process_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let desired_access = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let token_handle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
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
