use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn convert_fiber_to_thread(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!ConvertFiberToThread");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
