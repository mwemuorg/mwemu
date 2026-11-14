use crate::emu;

pub fn strftime(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let max_size = emu.regs().rdx as u32;
    let s = "01/01/2024 00:00:00";
    if dst != 0 && max_size > 0 {
        emu.maps.write_string(dst, s);
    }
    log_red!(emu, "msvcrt!strftime => '{}'", s);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = s.len() as u64;
}
