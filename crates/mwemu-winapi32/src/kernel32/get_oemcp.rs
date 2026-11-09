use crate::emu;

pub fn GetOEMCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetOEMCP");
    emu.regs_mut().rax = 0x00000409;
}
