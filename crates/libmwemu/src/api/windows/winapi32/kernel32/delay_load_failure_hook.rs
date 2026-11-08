use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DelayLoadFailureHook(emu: &mut emu::Emu) {
    let dll_name = arg(emu, 0) as u64;
    let p_delay_info = arg(emu, 1) as u64;

    let name = emu.maps.read_string(dll_name);
    log_red!(
        emu,
        "kernel32!DelayLoadFailureHook dll:'{}' pDelayInfo:0x{:x}",
        name,
        p_delay_info
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
