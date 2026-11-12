use crate::emu;
use crate::winapi::helper;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let process_handle = emu.regs().rcx;
    let _desired_access = emu.regs().rdx;
    let token_handle_ptr = emu.regs().r8;

    log_red!(emu, "kernel32!OpenProcessToken hndl:0x{:x}", process_handle);

    if token_handle_ptr != 0 {
        emu.maps.write_qword(
            token_handle_ptr,
            helper::handler_create(&format!("token://{}", process_handle)) as u64,
        );
    }

    emu.regs_mut().rax = 1; // TRUE
}
