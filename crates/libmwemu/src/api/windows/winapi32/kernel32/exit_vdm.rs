use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ExitVDM(emu: &mut emu::Emu) {
    let is_wow = arg(emu, 0) as u64;
    let exit_code = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!ExitVDM isWow:{} exitCode:{}",
        is_wow,
        exit_code
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    stdcall_ret(emu, 2);
}
