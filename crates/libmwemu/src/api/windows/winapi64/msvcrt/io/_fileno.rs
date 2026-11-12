use crate::emu;
use crate::winapi::helper;

pub fn _fileno(emu: &mut emu::Emu) {
    let stream = emu.regs().rcx;
    let fd = if helper::handler_exist(stream) {
        stream & 0xffff
    } else {
        0
    };
    log_red!(emu, "msvcrt!_fileno stream:0x{:x} => {}", stream, fd);
    emu.set_last_error(0);
    emu.regs_mut().rax = fd;
}
