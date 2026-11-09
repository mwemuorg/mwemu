use crate::emu;

pub fn GetCPInfo(emu: &mut emu::Emu) {
    let cp = emu.regs().rcx;
    let info_ptr = emu.regs().rdx;
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 1); // MaxCharSize
        emu.maps.write_byte(info_ptr + 4, 0x3f); // DefaultChar[0] = '?'
    }
    log_red!(emu, "kernelbase!GetCPInfo cp:{}", cp);
    emu.regs_mut().rax = 1;
}
