use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNextVDMCommand(emu: &mut emu::Emu) {
    let vdm_info_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetNextVDMCommand info:0x{:x} => 0",
        vdm_info_ptr
    );
    let _ = helper::handler_exist(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
