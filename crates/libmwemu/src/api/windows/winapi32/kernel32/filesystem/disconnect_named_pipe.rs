use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn DisconnectNamedPipe(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!DisconnectNamedPipe hndl: 0x{:x}", handle);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
