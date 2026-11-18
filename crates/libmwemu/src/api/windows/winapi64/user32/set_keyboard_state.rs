use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetKeyboardState(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!SetKeyboardState buf:0x{:x}", buf);
    if buf != 0 && emu.maps.is_mapped(buf) {
        let _ = emu.maps.read_dword(buf).unwrap_or(0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
