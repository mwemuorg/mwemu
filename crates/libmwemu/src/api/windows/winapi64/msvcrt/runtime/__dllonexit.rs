use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __dllonexit(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let func = api_try!(emu, call.arg(emu, 0));
    let p_begin = api_try!(emu, call.arg(emu, 1));
    let p_end = api_try!(emu, call.arg(emu, 2));
    if p_begin != 0 {
        let begin = emu.maps.read_qword(p_begin).unwrap_or(0);
        let end = if p_end != 0 {
            emu.maps.read_qword(p_end).unwrap_or(0)
        } else {
            0
        };
        log_red!(
            emu,
            "msvcrt!__dllonexit func:0x{:x} begin:0x{:x}(0x{:x}) end:0x{:x}(0x{:x})",
            func,
            p_begin,
            begin,
            p_end,
            end
        );
        let first_param = api_try!(emu, call.arg(emu, 0));
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
    } else {
        log_red!(
            emu,
            "msvcrt!__dllonexit func:0x{:x} begin:0x{:x} end:0x{:x}",
            func,
            p_begin,
            p_end
        );
    }
    emu.set_last_error(0);
    call.set_return(emu, func);
}
