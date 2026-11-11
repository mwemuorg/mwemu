use crate::emu;

pub fn InternetEnumPerSiteCookieDecisionA(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let buf_size = emu.regs().rdx;
    let index_ptr = emu.regs().r8;
    log_red!(
        emu,
        "wininet!InternetEnumPerSiteCookieDecisionA buf:0x{:x} idx:0x{:x}",
        buf_ptr,
        index_ptr
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(259);
    emu.regs_mut().rax = 0;
}

pub fn InternetEnumPerSiteCookieDecisionW(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let buf_size = emu.regs().rdx;
    let index_ptr = emu.regs().r8;
    log_red!(
        emu,
        "wininet!InternetEnumPerSiteCookieDecisionW buf:0x{:x} idx:0x{:x}",
        buf_ptr,
        index_ptr
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(259);
    emu.regs_mut().rax = 0;
}
