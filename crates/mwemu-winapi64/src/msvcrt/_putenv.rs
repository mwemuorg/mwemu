use crate::emu;

pub fn _putenv(emu: &mut emu::Emu, api: &str) {
    let ptr = emu.regs().rcx;
    let s = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}'", api, s);
    emu.regs_mut().rax = 0;
}
