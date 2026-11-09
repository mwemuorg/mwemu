use crate::emu;
use crate::winapi::helper;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let process_handle = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let token_handle_ptr = emu.regs().r8;
    let hndl = helper::handler_create("token://process");
    if token_handle_ptr != 0 {
        emu.maps.write_qword(token_handle_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!OpenProcessToken proc:0x{:x} access:0x{:x} => 0x{:x}",
        process_handle,
        desired_access,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
