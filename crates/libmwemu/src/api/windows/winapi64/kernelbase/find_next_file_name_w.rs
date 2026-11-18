use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindNextFileNameW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let name = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernelbase!FindNextFileNameW '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
