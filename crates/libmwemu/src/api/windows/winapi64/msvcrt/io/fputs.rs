use crate::emu;

pub fn fputs(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;
    let s = emu.maps.read_string(s_ptr);
    log_red!(emu, "msvcrt!fputs '{}'", s);
    emu.regs_mut().rax = 0;
}
