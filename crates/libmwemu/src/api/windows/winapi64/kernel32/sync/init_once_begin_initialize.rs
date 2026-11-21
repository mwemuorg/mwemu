use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitOnceBeginInitialize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lp_init_once = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg_u32(emu, 1));
    let f_pending_ptr = api_try!(emu, call.arg(emu, 2));
    let _lp_context = api_try!(emu, call.arg(emu, 3));

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
    call.set_return(emu, 1); // TRUE = success
}
