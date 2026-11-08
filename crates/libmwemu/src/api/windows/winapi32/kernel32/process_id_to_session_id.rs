use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ProcessIdToSessionId(emu: &mut emu::Emu) {
    let process_id = arg(emu, 0) as u64 as u32;
    let session_id_ptr = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!ProcessIdToSessionId pid:{} out:0x{:x}",
        process_id,
        session_id_ptr
    );

    if session_id_ptr != 0 {
        emu.maps.write_dword(session_id_ptr, 0); // session 0
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
