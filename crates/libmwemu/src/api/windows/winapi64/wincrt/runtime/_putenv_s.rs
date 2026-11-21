use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _putenv_s(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let value_ptr = api_try!(emu, call.arg(emu, 1));
    let name = emu.maps.read_string(name_ptr);
    let value = emu.maps.read_string(value_ptr);
    log_red!(emu, "wincrt!_putenv_s '{}={}'", name, value);
    call.set_return(emu, 0);
}
