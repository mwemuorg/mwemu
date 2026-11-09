use crate::emu;

pub fn wcscat_s(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let _dst_size = emu.regs().rdx;
    let src = emu.regs().r8;
    let existing = emu.maps.read_wide_string(dst);
    let append = emu.maps.read_wide_string(src);
    let result = format!("{}{}", existing, append);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &result);
    }
    log_red!(emu, "msvcrt!wcscat_s => '{}'", result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
