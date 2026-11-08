use crate::emu;

pub fn CheckTokenMembership(emu: &mut emu::Emu) {
    let token = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sid_to_check = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let is_member_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    if is_member_ptr != 0 {
        emu.maps.write_dword(is_member_ptr, 1); // TRUE - user is member
    }
    log_red!(
        emu,
        "advapi32!CheckTokenMembership token:0x{:x} sid:0x{:x} => TRUE",
        token,
        sid_to_check
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
