use crate::emu;

pub fn SetSecurityDescriptorGroup(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let group_ptr = emu.regs().rdx;
    let defaulted = emu.regs().r8 != 0;
    if sd_ptr != 0 {
        emu.maps.write_qword(sd_ptr + 20, group_ptr);
        let mut control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if defaulted {
            control |= 0x0002;
        } else {
            control &= !0x0002;
        }
        emu.maps.write_word(sd_ptr + 2, control);
    }
    log_red!(
        emu,
        "kernelbase!SetSecurityDescriptorGroup sd:0x{:x} group:0x{:x}",
        sd_ptr,
        group_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
