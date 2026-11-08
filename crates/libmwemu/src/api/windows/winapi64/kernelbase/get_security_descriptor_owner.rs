use crate::emu;

pub fn GetSecurityDescriptorOwner(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let owner_ptr_ptr = emu.regs().rdx;
    let defaulted_ptr = emu.regs().r8;
    if sd_ptr != 0 {
        let owner = emu.maps.read_qword(sd_ptr + 12).unwrap_or(0);
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if owner_ptr_ptr != 0 {
            emu.maps.write_qword(owner_ptr_ptr, owner);
        }
        if defaulted_ptr != 0 {
            let defaulted = (control & 0x0001) != 0; // SE_OWNER_DEFAULTED
            emu.maps
                .write_dword(defaulted_ptr, if defaulted { 1 } else { 0 });
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorOwner sd:0x{:x}",
        sd_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
