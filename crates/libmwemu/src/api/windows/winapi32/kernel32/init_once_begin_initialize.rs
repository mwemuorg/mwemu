use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn InitOnceBeginInitialize(emu: &mut emu::Emu) {
    let lp_init_once = arg(emu, 0) as u64;
    let dw_flags = arg(emu, 1);
    let f_pending_ptr = arg(emu, 2) as u64;
    let _lp_context = arg(emu, 3) as u64;

    const INIT_ONCE_CHECK_ONLY: u32 = 0x00000001;
    const INIT_ONCE_ASYNC: u32 = 0x00000002;

    log_red!(
        emu,
        "kernel32!InitOnceBeginInitialize once:0x{:x} flags:0x{:x}",
        lp_init_once,
        dw_flags
    );

    if lp_init_once != 0 {
        let state = emu.maps.read_qword(lp_init_once).unwrap_or(0);
        if state == 0 {
            // Not yet initialized — mark as in-progress and set fPending=TRUE
            emu.maps.write_qword(lp_init_once, 1);
            if f_pending_ptr != 0 {
                emu.maps.write_dword(f_pending_ptr, 1); // TRUE = needs initialization
            }
        } else {
            // Already initialized — set fPending=FALSE
            if f_pending_ptr != 0 {
                emu.maps.write_dword(f_pending_ptr, 0); // FALSE = already done
            }
        }
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE = success
    stdcall_ret(emu, 4);
}
