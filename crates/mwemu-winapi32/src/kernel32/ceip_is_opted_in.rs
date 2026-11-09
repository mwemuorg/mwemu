use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn ceip_is_opted_in(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CeipIsOptedIn");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
