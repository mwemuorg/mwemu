use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wrename(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let old_ptr = api_try!(emu, call.arg(emu, 0));
    let new_ptr = api_try!(emu, call.arg(emu, 1));
    let old_name = emu.maps.read_wide_string(old_ptr);
    let new_name = emu.maps.read_wide_string(new_ptr);
    log_red!(emu, "msvcrt!_wrename '{}' => '{}'", old_name, new_name);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
