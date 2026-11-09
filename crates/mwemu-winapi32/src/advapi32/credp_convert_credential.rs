use crate::emu;

pub fn CredpConvertCredential(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!CredpConvertCredential arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
