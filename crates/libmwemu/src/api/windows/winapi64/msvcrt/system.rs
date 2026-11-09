use crate::emu;

pub fn system(emu: &mut emu::Emu) {
    let cmd_ptr = emu.regs().rcx;
    let cmd = emu.maps.read_string(cmd_ptr);
    log_red!(emu, "msvcrt!system '{}'", cmd);
    emu.regs_mut().rax = 0;
}
