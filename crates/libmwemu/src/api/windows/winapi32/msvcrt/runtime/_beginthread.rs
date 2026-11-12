use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn _beginthread(emu: &mut emu::Emu) {
    let start = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!_beginthread start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthread");
    emu.regs_mut().rax = hndl;
}
