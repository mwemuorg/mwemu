use crate::emu;

pub fn strcat(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let existing = emu.maps.read_string(dst);
    let append = emu.maps.read_string(src);
    let result = format!("{}{}", existing, append);
    if dst != 0 {
        emu.maps.write_string(dst, &result);
    }
    log_red!(emu, "msvcrt!strcat => '{}'", result);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = dst;
}
