use crate::emu;

pub fn strcat(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let existing = emu.maps.read_string(dst);
    let append = emu.maps.read_string(src);
    let result = format!("{}{}", existing, append);
    if dst != 0 {
        emu.maps.write_string(dst, &result);
    }
    log_red!(emu, "msvcrt!strcat => '{}'", result);
    emu.set_last_error(0);
    emu.regs_mut().rax = dst;
}
