use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ImpersonateLoggedOnUser(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let htoken = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "advapi32!ImpersonateLoggedOnUser hToken:0x{:x}",
        htoken
    );
    let result = if helper::handler_exist(htoken) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    call.set_return(emu, result);
}
