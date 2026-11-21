use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _XcptFilter(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let exc_code = api_try!(emu, call.arg(emu, 0));
    let exc_ptrs = api_try!(emu, call.arg(emu, 1));
    if exc_ptrs != 0 {
        let exc_record = emu.maps.read_qword(exc_ptrs).unwrap_or(0);
        let ctx_record = emu.maps.read_qword(exc_ptrs + 8).unwrap_or(0);
        log_red!(
            emu,
            "msvcrt!_XcptFilter code:0x{:x} ptrs:0x{:x} rec:0x{:x} ctx:0x{:x}",
            exc_code,
            exc_ptrs,
            exc_record,
            ctx_record
        );
        let first_param = api_try!(emu, call.arg(emu, 0));
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
    } else {
        log_red!(emu, "msvcrt!_XcptFilter code:0x{:x} ptrs:NULL", exc_code);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1); // EXCEPTION_EXECUTE_HANDLER
}
