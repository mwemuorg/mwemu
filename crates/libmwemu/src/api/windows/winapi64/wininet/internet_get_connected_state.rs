use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetGetConnectedState(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wininet!InternetGetConnectedState");
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x20);
    }
    call.set_return(emu, 1);
}
