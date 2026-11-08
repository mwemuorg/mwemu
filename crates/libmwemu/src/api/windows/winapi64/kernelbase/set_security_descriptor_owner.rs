use crate::emu;

pub fn SetSecurityDescriptorOwner(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let owner_ptr = emu.regs().rdx;
    let defaulted = emu.regs().r8 != 0;
    if sd_ptr != 0 {
        emu.maps.write_qword(sd_ptr + 12, owner_ptr);
        let mut control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if defaulted {
            control |= 0x0001;
        } else {
            control &= !0x0001;
        }
        emu.maps.write_word(sd_ptr + 2, control);
    }
    log_red!(
        emu,
        "kernelbase!SetSecurityDescriptorOwner sd:0x{:x} owner:0x{:x}",
        sd_ptr,
        owner_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
