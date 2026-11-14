use crate::emu;

pub fn printf(emu: &mut emu::Emu) {
    let fmt_ptr = emu.regs().rcx;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!printf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
