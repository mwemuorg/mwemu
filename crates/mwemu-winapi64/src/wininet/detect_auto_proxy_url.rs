use crate::emu;

pub fn DetectAutoProxyUrl(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!DetectAutoProxyUrl arg1:0x{:x}", arg1);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}
