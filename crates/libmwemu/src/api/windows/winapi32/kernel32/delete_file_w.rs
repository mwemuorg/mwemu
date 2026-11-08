use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DeleteFileW(emu: &mut emu::Emu) {
    let filename = emu.maps.read_wide_string(arg(emu, 0) as u64);
    log_red!(
        emu,
        "kernel32!DeleteFileW `{}` (stub: not deleting)",
        filename
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
