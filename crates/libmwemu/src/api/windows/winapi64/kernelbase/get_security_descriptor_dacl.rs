use crate::emu;

pub fn GetSecurityDescriptorDacl(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let dacl_present_ptr = emu.regs().rdx;
    let dacl_ptr_ptr = emu.regs().r8;
    let defaulted_ptr = emu.regs().r9;
    if sd_ptr != 0 {
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        let has_dacl = (control & 0x0004) != 0;
        let dacl = emu.maps.read_qword(sd_ptr + 28).unwrap_or(0);
        if dacl_present_ptr != 0 {
            emu.maps
                .write_dword(dacl_present_ptr, if has_dacl { 1 } else { 0 });
        }
        if dacl_ptr_ptr != 0 {
            emu.maps
                .write_qword(dacl_ptr_ptr, if has_dacl { dacl } else { 0 });
        }
        if defaulted_ptr != 0 {
            let defaulted = (control & 0x0008) != 0;
            emu.maps
                .write_dword(defaulted_ptr, if defaulted { 1 } else { 0 });
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorDacl sd:0x{:x}",
        sd_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
