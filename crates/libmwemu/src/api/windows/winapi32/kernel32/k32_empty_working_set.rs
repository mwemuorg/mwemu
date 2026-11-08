use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn K32EmptyWorkingSet(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!K32EmptyWorkingSet hProcess:0x{:x}",
        h_process
    );
    let _ = helper::handler_exist(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
