use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DeleteFileFromAppW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = emu.maps.read_wide_string(path_ptr);
    log_red!(emu, "kernelbase!DeleteFileFromAppW '{}'", path);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
