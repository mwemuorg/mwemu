use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn PostQueuedCompletionStatus(emu: &mut emu::Emu) {
    let h_port = arg(emu, 0) as u64;
    let num_bytes = arg(emu, 1) as u64 as u32;
    let comp_key = arg(emu, 2) as u64;
    let p_overlapped = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!PostQueuedCompletionStatus hPort:0x{:x} bytes:{} key:0x{:x} pOvl:0x{:x}",
        h_port,
        num_bytes,
        comp_key,
        p_overlapped
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
