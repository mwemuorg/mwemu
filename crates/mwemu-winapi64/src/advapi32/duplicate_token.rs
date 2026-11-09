use crate::emu;
use crate::winapi::helper;

pub fn DuplicateToken(emu: &mut emu::Emu) {
    let existing_token = emu.regs().rcx;
    let _impersonation_level = emu.regs().rdx;
    let dup_token_ptr = emu.regs().r8;
    let new_token = helper::handler_create("dup_token");
    if dup_token_ptr != 0 {
        emu.maps.write_qword(dup_token_ptr, new_token);
    }
    log_red!(
        emu,
        "advapi32!DuplicateToken src:0x{:x} => 0x{:x}",
        existing_token,
        new_token
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
