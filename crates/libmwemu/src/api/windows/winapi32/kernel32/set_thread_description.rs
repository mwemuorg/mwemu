use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetThreadDescription(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    let description_ptr = arg(emu, 1) as u64;
    let desc = emu.maps.read_wide_string(description_ptr);
    log_red!(
        emu,
        "kernel32!SetThreadDescription hThread:0x{:x} desc:'{}'",
        h_thread,
        desc
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
