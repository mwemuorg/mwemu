use crate::emu;

pub fn CryptFormatObject(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptFormatObject arg1:0x{:x}", arg1);
    let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
