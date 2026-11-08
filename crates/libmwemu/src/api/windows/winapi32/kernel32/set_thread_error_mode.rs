use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetThreadErrorMode(emu: &mut emu::Emu) {
    let new_mode = arg(emu, 0) as u64 as u32;
    let p_old_mode = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!SetThreadErrorMode mode:0x{:x} pOld:0x{:x}",
        new_mode,
        p_old_mode
    );
    if p_old_mode != 0 {
        emu.maps.write_dword(p_old_mode, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
