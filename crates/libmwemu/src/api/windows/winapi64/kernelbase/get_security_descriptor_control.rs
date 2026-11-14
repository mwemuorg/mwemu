use crate::emu;

pub fn GetSecurityDescriptorControl(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let control_ptr = emu.regs().rdx;
    let revision_ptr = emu.regs().r8;
    if sd_ptr != 0 {
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if control_ptr != 0 {
            emu.maps.write_word(control_ptr, control);
        }
        if revision_ptr != 0 {
            let rev = emu.maps.read_byte(sd_ptr).unwrap_or(1);
            emu.maps.write_dword(revision_ptr, rev as u32);
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorControl sd:0x{:x}",
        sd_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
