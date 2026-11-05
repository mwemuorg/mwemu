use crate::emu;

pub fn CheckTokenMembershipEx(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rcx;
    let sid_to_check = emu.regs().rdx;
    let flags = emu.regs().r8;
    let is_member = emu.regs().r9;
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
}
