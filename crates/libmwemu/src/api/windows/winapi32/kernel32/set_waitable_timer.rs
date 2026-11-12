use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetWaitableTimer(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "kernel32!SetWaitableTimer 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
