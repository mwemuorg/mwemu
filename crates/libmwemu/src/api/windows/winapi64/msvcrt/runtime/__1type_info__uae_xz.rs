use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __1type_info__UAE_XZ(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let this_ptr = api_try!(emu, call.arg(emu, 0));
    let vtable = if this_ptr != 0 {
        emu.maps.read_qword(this_ptr).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "msvcrt!~type_info this:0x{:x} vtable:0x{:x}",
        this_ptr,
        vtable
    );
}
