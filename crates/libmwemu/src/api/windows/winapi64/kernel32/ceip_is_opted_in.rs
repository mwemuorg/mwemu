use crate::emu;

pub fn CeipIsOptedIn(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CeipIsOptedIn");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
