use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn DuplicateTokenEx(emu: &mut emu::Emu) {
    let existing_token = emu.regs().rcx;
    let _desired_access = emu.regs().rdx;
    let _token_attributes = emu.regs().r8;
    let _impersonation_level = emu.regs().r9;
    let _token_type = WinApiAbi::arg(emu, 4);
    let dup_token_ptr = WinApiAbi::arg(emu, 5);
    let new_token = helper::handler_create("dup_token_ex");
    if dup_token_ptr != 0 {
        emu.maps.write_qword(dup_token_ptr, new_token);
    }
    log_red!(
        emu,
        "advapi32!DuplicateTokenEx src:0x{:x} => 0x{:x}",
        existing_token,
        new_token
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
