use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _setmode(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg(emu, 0));
    let mode = api_try!(emu, call.arg(emu, 1));
    let prev_mode = 0x4000u64; // _O_TEXT
    let name = helper::handler_get_uri(fd);
    log_red!(
        emu,
        "msvcrt!_setmode fd:{} '{}' mode:0x{:x} => 0x{:x}",
        fd,
        name,
        mode,
        prev_mode
    );
    emu.set_last_error(0);
    call.set_return(emu, prev_mode);
}
