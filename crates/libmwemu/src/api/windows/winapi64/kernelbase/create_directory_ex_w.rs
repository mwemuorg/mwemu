use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CreateDirectoryExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let security = api_try!(emu, call.arg(emu, 1));
    let path = emu.maps.read_wide_string(path_ptr);
    log_red!(emu, "kernelbase!CreateDirectoryExW '{}'", path);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
