use crate::emu;
use crate::winapi::helper;

pub fn _unlock(emu: &mut emu::Emu) {
    let lock_num = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!_unlock {}", lock_num);
    helper::advance_tick(emu, 1);
}
