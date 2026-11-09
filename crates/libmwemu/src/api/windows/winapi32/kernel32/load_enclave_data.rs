use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn LoadEnclaveData(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let lp_address = arg(emu, 1) as u64;
    let lp_buffer = arg(emu, 2) as u64;
    let n_size = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!LoadEnclaveData hProc:0x{:x} addr:0x{:x} buf:0x{:x} size:{}",
        h_process,
        lp_address,
        lp_buffer,
        n_size
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
