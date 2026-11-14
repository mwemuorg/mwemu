use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn _unlock(emu: &mut emu::Emu) {
    let lock_num = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!_unlock {}", lock_num);
    helper::advance_tick(emu, 1);
}
