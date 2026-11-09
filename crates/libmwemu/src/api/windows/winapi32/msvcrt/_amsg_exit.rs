use crate::emu;

pub fn _amsg_exit(emu: &mut emu::Emu) {
    let code = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!_amsg_exit code:{}", code);
    emu.stop();
}
