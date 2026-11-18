use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetKeyboardState(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    if buf != 0 {
        for i in 0..256u64 {
            emu.maps.write_byte(buf + i, 0);
        }
    }
    log_red!(emu, "user32!GetKeyboardState buf:0x{:x}", buf);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
