use crate::emu;

pub fn GetCPInfoExW(emu: &mut emu::Emu) {
    let cp = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let info_ptr = emu.regs().r8;
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 1);
        emu.maps.write_byte(info_ptr + 4, 0x3f);
    }
    log_red!(emu, "kernelbase!GetCPInfoExW cp:{} flags:0x{:x}", cp, flags);
    emu.regs_mut().rax = 1;
}
