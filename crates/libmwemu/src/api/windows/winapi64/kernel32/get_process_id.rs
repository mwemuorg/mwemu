use crate::emu;

pub fn GetProcessId(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!GetProcessId h:0x{:x} => 1337", handle);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1337;
}
