use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenRegStream(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let value_ptr = api_try!(emu, call.arg(emu, 2));
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "shell32!OpenRegStream hkey:0x{:x} '{}'", hkey, subkey);
    let hndl = helper::handler_create("shell32://OpenRegStream");
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
