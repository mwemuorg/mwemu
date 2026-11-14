use crate::emu;

pub fn SetSecurityDescriptorDacl(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let dacl_present = emu.regs().rdx != 0;
    let dacl_ptr = emu.regs().r8;
    let defaulted = emu.regs().r9 != 0;
    if sd_ptr != 0 {
        let mut control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if dacl_present {
            control |= 0x0004;
        } else {
            control &= !0x0004;
        }
        if defaulted {
            control |= 0x0008;
        } else {
            control &= !0x0008;
        }
        emu.maps.write_word(sd_ptr + 2, control);
        emu.maps.write_qword(sd_ptr + 28, dacl_ptr);
    }
    log_red!(
        emu,
        "kernelbase!SetSecurityDescriptorDacl sd:0x{:x} present:{} dacl:0x{:x}",
        sd_ptr,
        dacl_present,
        dacl_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
