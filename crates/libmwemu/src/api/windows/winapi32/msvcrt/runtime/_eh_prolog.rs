use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _EH_prolog(emu: &mut emu::Emu) {
    let handler = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!_EH_prolog handler:0x{:x}", handler);
    emu.stack_push32(emu.regs().get_ebp() as u32);
}
