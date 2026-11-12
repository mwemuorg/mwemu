use crate::emu;

pub fn system(emu: &mut emu::Emu) {
    let cmd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let cmd = emu.maps.read_string(cmd_ptr);
    log_red!(emu, "msvcrt!system '{}'", cmd);
    emu.regs_mut().rax = 0;
}
