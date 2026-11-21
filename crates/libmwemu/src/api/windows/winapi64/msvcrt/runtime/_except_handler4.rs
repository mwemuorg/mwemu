use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _except_handler4(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let exc_record = api_try!(emu, call.arg(emu, 0));
    let frame_ptr = api_try!(emu, call.arg(emu, 1));
    let exc_code = if exc_record != 0 {
        emu.maps.read_dword(exc_record).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "msvcrt!{} exc:0x{:x} code:0x{:x} frame:0x{:x}",
        api,
        exc_record,
        exc_code,
        frame_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
