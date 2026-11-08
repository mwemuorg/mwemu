use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn EscapeCommFunction(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let func = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!EscapeCommFunction hFile:0x{:x} func:{}",
        h_file,
        func
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
