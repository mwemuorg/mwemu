use crate::emu;

pub fn CertSrvIsServerOnlineW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "crypt32!CertSrvIsServerOnlineW '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
