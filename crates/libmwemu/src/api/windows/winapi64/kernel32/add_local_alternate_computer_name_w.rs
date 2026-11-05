use crate::emu;
use crate::windows::constants;

pub fn AddLocalAlternateComputerNameW(emu: &mut emu::Emu) {
    let Hostname = emu.regs().rcx as u32;
    let _a2 = emu.regs().rdx as u32;

    let Hostname = emu.maps.read_wide_string(Hostname as u64);

    log_red!(emu, "kernel32!AddLocalAlternateComputerNameW {}", Hostname);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
