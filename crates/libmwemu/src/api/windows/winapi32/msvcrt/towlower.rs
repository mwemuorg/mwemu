use crate::emu;

pub fn towlower(emu: &mut emu::Emu) {
    let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let ch = char::from_u32(c).unwrap_or('\0');
    let lower = ch.to_lowercase().next().unwrap_or(ch);
    log_red!(emu, "msvcrt!towlower '{}' => '{}'", ch, lower);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = lower as u32 as u64;
}
