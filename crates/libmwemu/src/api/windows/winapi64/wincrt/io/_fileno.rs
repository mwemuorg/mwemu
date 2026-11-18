use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _fileno(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let stream = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!_fileno stream:0x{:x}", stream);
    let _ = emu.maps.is_mapped(stream);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
