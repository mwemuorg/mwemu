use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _EH_prolog3(emu: &mut emu::Emu, api: &str) {
    let cookie = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!{} cookie:0x{:x}", api, cookie);
    emu.stack_push32(emu.regs().get_ebp() as u32);
}
