use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hKey = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!RegCloseKey hkey: 0x{:x}", hKey);
    call.cleanup_args(emu, 1);
    call.set_return(emu, ERROR_SUCCESS);
}
