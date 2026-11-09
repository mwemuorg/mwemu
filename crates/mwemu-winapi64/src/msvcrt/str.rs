use crate::emu;

pub fn str(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;
    let s = emu.maps.read_string(s_ptr);
    log_red!(emu, "msvcrt!str 0x{:x} '{}'", s_ptr, s);
    emu.regs_mut().rax = s_ptr;
}
