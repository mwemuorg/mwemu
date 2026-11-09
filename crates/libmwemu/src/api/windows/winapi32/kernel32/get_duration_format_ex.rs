use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetDurationFormatEx(emu: &mut emu::Emu) {
    let locale_ptr = arg(emu, 0) as u64;
    let flags = arg(emu, 1);
    let duration_ptr = arg(emu, 2) as u64;
    let duration_val = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetDurationFormatEx locale:0x{:x} flags:0x{:x}",
        locale_ptr,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
