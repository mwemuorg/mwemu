use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptGetDefaultOIDFunctionAddress(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "crypt32!CryptGetDefaultOIDFunctionAddress arg1:0x{:x}",
        arg1
    );
    let handle = helper::handler_create("CryptGetDefaultOIDFunctionAddress");
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
