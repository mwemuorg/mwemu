use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn _beginthreadex(emu: &mut emu::Emu) {
    let _security = WinApiAbi::arg(emu, 0) as u32;
    let _stack_size = WinApiAbi::arg(emu, 1) as u32;
    let start = WinApiAbi::arg(emu, 2) as u32;
    log_red!(emu, "msvcrt!_beginthreadex start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthreadex");
    emu.regs_mut().rax = hndl;
}
