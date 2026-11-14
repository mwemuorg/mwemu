use crate::emu;

pub fn WEP(emu: &mut emu::Emu) {
    let reason = emu.regs().rcx;
    log_red!(emu, "ws2_32!WEP reason:{}", reason);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
