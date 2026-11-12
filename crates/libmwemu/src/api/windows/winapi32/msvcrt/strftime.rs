use crate::emu;

pub fn strftime(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let max_size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let s = "01/01/2024 00:00:00";
    if dst != 0 && max_size > 0 {
        emu.maps.write_string(dst, s);
    }
    log_red!(emu, "msvcrt!strftime => '{}'", s);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = s.len() as u64;
}
