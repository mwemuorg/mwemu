use crate::emu;

pub fn sprintf(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let fmt = emu.maps.read_string(fmt_ptr);
    if buf != 0 {
        emu.maps.write_string(buf, &fmt);
    }
    log_red!(emu, "msvcrt!sprintf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
