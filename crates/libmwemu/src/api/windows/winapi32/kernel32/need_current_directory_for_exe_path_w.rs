use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn NeedCurrentDirectoryForExePathW(emu: &mut emu::Emu) {
    let exe_name_ptr = arg(emu, 0) as u64;
    let exe_name = emu.maps.read_wide_string(exe_name_ptr);
    log_red!(
        emu,
        "kernel32!NeedCurrentDirectoryForExePathW '{}'",
        exe_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
