use crate::emu;

pub fn InitializeSecurityDescriptor(emu: &mut emu::Emu) {
    let sd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let revision = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    if sd_ptr != 0 {
        emu.maps.write_byte(sd_ptr, revision as u8);
        emu.maps.write_byte(sd_ptr + 1, 0); // Sbz1
        emu.maps.write_word(sd_ptr + 2, 0); // Control
        emu.maps.write_dword(sd_ptr + 4, 0); // Owner
        emu.maps.write_dword(sd_ptr + 8, 0); // Group
        emu.maps.write_dword(sd_ptr + 12, 0); // Sacl
        emu.maps.write_dword(sd_ptr + 16, 0); // Dacl
    }
    log_red!(
        emu,
        "advapi32!InitializeSecurityDescriptor sd:0x{:x} rev:{}",
        sd_ptr,
        revision
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
