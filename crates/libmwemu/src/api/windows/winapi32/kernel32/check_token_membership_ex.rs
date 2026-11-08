use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CheckTokenMembershipEx(emu: &mut emu::Emu) {
    let token_handle = arg(emu, 0) as u64;
    let sid_to_check = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let is_member = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!CheckTokenMembershipEx token:0x{:x} sid:0x{:x} flags:0x{:x} isMember:0x{:x}",
        token_handle,
        sid_to_check,
        flags,
        is_member
    );
    if is_member != 0 {
        emu.maps.write_dword(is_member, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
