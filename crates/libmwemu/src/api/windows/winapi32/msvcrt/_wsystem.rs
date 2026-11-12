use crate::emu;

pub fn _wsystem(emu: &mut emu::Emu) {
    let cmd_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let cmd = emu.maps.read_wide_string(cmd_ptr);
    log_red!(emu, "msvcrt!_wsystem cmd:'{}'", cmd);
    emu.regs_mut().rax = 0;
}
