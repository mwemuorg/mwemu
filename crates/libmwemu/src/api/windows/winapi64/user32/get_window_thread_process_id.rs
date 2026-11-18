use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetWindowThreadProcessId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let pid_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!GetWindowThreadProcessId hwnd:0x{:x}", hwnd);
    if pid_ptr != 0 {
        emu.maps.write_dword(pid_ptr, 0x123);
    }
    call.set_return(emu, 0x123);
}
