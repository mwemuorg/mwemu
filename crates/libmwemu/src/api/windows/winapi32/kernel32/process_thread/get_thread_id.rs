use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetThreadId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 1));

    call.cleanup_args(emu, 1);

    for i in 0..emu.threads.len() {
        if emu.threads[i].handle == hndl {
            let return_value = emu.threads[i].id;
            call.set_return(emu, return_value);
            log_red!(
                emu,
                "kernel32!GetThreadId hndl:{} (requested handle exists and its tid {})",
                hndl,
                emu.threads[i].id
            );
            return;
        }
    }
    log_red!(
        emu,
        "kernel32!GetThreadId hndl:{} (requested handle doesn't exist, returning a fake handle for now but should return zero.)",
        hndl
    );
    call.set_return(emu, 0x2c2878); // if handle not found should return zero.
}
