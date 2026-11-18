use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetGetConnectedStateEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wininet!InternetGetConnectedStateEx");
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x20);
    }
    call.set_return(emu, 1);
}

pub fn InternetGetConnectedStateExA(emu: &mut emu::Emu) {
    internet_get_connected_state_ex(emu, "InternetGetConnectedStateExA");
}

fn internet_get_connected_state_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let flags_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wininet!{api}");
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x20);
    }
    call.set_return(emu, 1);
}

pub fn InternetGetConnectedStateExW(emu: &mut emu::Emu) {
    internet_get_connected_state_ex(emu, "InternetGetConnectedStateExW");
}
