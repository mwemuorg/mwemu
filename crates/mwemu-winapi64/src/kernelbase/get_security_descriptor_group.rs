use crate::emu;

pub fn GetSecurityDescriptorGroup(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let group_ptr_ptr = emu.regs().rdx;
    let defaulted_ptr = emu.regs().r8;
    if sd_ptr != 0 {
        let group = emu.maps.read_qword(sd_ptr + 20).unwrap_or(0);
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if group_ptr_ptr != 0 {
            emu.maps.write_qword(group_ptr_ptr, group);
        }
        if defaulted_ptr != 0 {
            let defaulted = (control & 0x0002) != 0; // SE_GROUP_DEFAULTED
            emu.maps
                .write_dword(defaulted_ptr, if defaulted { 1 } else { 0 });
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorGroup sd:0x{:x}",
        sd_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
