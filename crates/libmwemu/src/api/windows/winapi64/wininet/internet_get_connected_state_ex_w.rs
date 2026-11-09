use crate::emu;

pub fn InternetGetConnectedStateExW(emu: &mut emu::Emu) {
    let flags_ptr = emu.regs().rcx;
    log_red!(emu, "wininet!InternetGetConnectedStateExW");
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x20);
    }
    emu.regs_mut().rax = 1;
}
