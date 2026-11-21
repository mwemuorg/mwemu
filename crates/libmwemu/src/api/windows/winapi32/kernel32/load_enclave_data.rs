use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn LoadEnclaveData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!LoadEnclaveData arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    call.cleanup_args(emu, 9);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
