use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsApiSetImplemented(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "kernelbase!IsApiSetImplemented '{}' => TRUE", name);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
