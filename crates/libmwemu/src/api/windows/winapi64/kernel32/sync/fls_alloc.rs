use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
pub fn FlsAlloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let callback = api_try!(emu, call.arg(emu, 0));
    let idx = emu.fls().len() as u64;
    emu.fls_mut().push(0);
    log_red!(
        emu,
        "kernel32!FlsAlloc callback:0x{:x} => idx:{}",
        callback,
        idx
    );
    let handle = helper::handler_create("FlsAlloc");
    emu.set_last_error(0);
    call.set_return(emu, idx);
}
