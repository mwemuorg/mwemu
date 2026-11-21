use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __CppXcptFilter(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let exc_num = api_try!(emu, call.arg(emu, 0));
    let exc_ptrs = api_try!(emu, call.arg(emu, 1));
    if exc_ptrs != 0 {
        let exc_record = emu.maps.read_qword(exc_ptrs).unwrap_or(0);
        log_red!(
            emu,
            "msvcrt!__CppXcptFilter exc:0x{:x} ptrs:0x{:x} rec:0x{:x}",
            exc_num,
            exc_ptrs,
            exc_record
        );
        let first_param = api_try!(emu, call.arg(emu, 0));
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
    } else {
        log_red!(emu, "msvcrt!__CppXcptFilter exc:0x{:x} ptrs:NULL", exc_num);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1); // EXCEPTION_EXECUTE_HANDLER
}
