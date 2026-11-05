use crate::emu;
use crate::winapi::helper;

pub fn CloseThreadpoolIo(emu: &mut emu::Emu) {
    let pio = emu.regs().rcx;
    log_red!(emu, "kernel32!CloseThreadpoolIo pio:0x{:x}", pio);
    helper::handler_close(pio);
}
