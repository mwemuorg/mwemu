use crate::emu;

pub fn CheckTokenMembership(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rcx;
    let _sid_to_check = emu.regs().rdx;
    let is_member_ptr = emu.regs().r8;
    if is_member_ptr != 0 {
        emu.maps.write_dword(is_member_ptr, 1);
    }
    log_red!(
        emu,
        "advapi32!CheckTokenMembership token:0x{:x} => member=TRUE",
        token_handle
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
