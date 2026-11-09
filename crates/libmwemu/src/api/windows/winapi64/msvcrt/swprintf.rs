use crate::emu;

pub fn swprintf(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    if buf != 0 {
        emu.maps.write_wide_string(buf, &fmt);
    }
    log_red!(emu, "msvcrt!swprintf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
