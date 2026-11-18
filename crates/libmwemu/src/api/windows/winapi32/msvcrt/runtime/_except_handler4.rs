use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _except_handler4(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let exc_record = api_try!(emu, call.arg_u32(emu, 0));
    let frame_ptr = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "msvcrt!{} exc:0x{:x} frame:0x{:x}",
        api,
        exc_record,
        frame_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // ExceptionContinueSearch
}
