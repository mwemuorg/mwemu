use crate::emu;

pub fn SetSecurityDescriptorSacl(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let sacl_present = emu.regs().rdx != 0;
    let sacl_ptr = emu.regs().r8;
    let defaulted = emu.regs().r9 != 0;
    if sd_ptr != 0 {
        let mut control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if sacl_present {
            control |= 0x0010;
        } else {
            control &= !0x0010;
        }
        if defaulted {
            control |= 0x0020;
        } else {
            control &= !0x0020;
        }
        emu.maps.write_word(sd_ptr + 2, control);
        emu.maps.write_qword(sd_ptr + 20, sacl_ptr);
    }
    log_red!(
        emu,
        "kernelbase!SetSecurityDescriptorSacl sd:0x{:x} present:{} sacl:0x{:x}",
        sd_ptr,
        sacl_present,
        sacl_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
