use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn InitializeEnclave(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let lp_address = arg(emu, 1) as u64;
    let lp_info = arg(emu, 2) as u64;
    let info_length = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!InitializeEnclave hProc:0x{:x} addr:0x{:x} info:0x{:x} len:{}",
        h_process,
        lp_address,
        lp_info,
        info_length
    );
    let handle = helper::handler_create("InitializeEnclave");
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
