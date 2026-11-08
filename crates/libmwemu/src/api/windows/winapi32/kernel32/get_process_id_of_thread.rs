use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessIdOfThread(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessIdOfThread h:0x{:x} => 0x123",
        h_thread
    );
    let _ = helper::handler_exist(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x123;
    stdcall_ret(emu, 1);
}
