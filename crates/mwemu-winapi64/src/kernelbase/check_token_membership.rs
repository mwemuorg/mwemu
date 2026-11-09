use crate::emu;

pub fn CheckTokenMembership(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let sid_to_check = emu.regs().rdx;
    let is_member = emu.regs().r8;
    log_red!(
        emu,
        "kernelbase!CheckTokenMembership token:0x{:x} sid:0x{:x}",
        token,
        sid_to_check
    );
    if is_member != 0 {
        emu.maps.write_dword(is_member, 1); // TRUE - is a member
    }
    emu.regs_mut().rax = 1; // success
}
