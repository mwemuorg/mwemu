use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn InternetSetStatusCallback(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let callback = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let valid = helper::handler_exist(handle);
    log_red!(
        emu,
        "wininet!InternetSetStatusCallback handle: 0x{:x} callback: 0x{:x} valid: {}",
        handle,
        callback,
        valid
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}
