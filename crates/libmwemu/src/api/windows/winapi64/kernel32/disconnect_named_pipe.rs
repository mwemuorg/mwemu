use crate::emu;

pub fn DisconnectNamedPipe(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;

    log_red!(emu, "kernel32!DisconnectNamedPipe hndl: 0x{:x}", handle);

    emu.regs_mut().rax = 1;
}
