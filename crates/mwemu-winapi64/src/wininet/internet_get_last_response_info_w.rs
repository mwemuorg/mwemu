use crate::emu;

pub fn InternetGetLastResponseInfoW(emu: &mut emu::Emu) {
    let error_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let buf_len_ptr = emu.regs().r8;
    log_red!(emu, "wininet!InternetGetLastResponseInfoW");
    if error_ptr != 0 {
        emu.maps.write_dword(error_ptr, 0);
    }
    if buf_len_ptr != 0 {
        emu.maps.write_dword(buf_len_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
