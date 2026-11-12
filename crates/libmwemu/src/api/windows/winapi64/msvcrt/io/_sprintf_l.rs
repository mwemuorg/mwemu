use crate::emu;

pub fn _sprintf_l(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let _locale = emu.regs().r8;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!_sprintf_l buf:0x{:x} fmt:'{}'", buf, fmt);
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }
    emu.regs_mut().rax = fmt.len() as u64;
}
