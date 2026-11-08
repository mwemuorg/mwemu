use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CtrlRoutine(emu: &mut emu::Emu) {
    let ctrl_type = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!CtrlRoutine ctrlType:{}", ctrl_type);
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    stdcall_ret(emu, 1);
}
