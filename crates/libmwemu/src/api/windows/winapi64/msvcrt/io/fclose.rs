use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn fclose(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    let filename = helper::handler_get_uri(file);
    helper::handler_close(file);
    log_red!(emu, "msvcrt!fclose '{}' handle:0x{:x}", filename, file);
    call.set_return(emu, 0);
}
