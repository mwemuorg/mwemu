use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNextUmsListItem(emu: &mut emu::Emu) {
    let ums_context = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetNextUmsListItem ctx:0x{:x} => 0 (end of list)",
        ums_context
    );
    let _ = helper::handler_exist(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
