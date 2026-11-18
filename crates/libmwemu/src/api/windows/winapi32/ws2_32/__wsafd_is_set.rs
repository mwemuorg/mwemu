use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __WSAFDIsSet(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let fd = api_try!(emu, call.arg(emu, 0));
    let set_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ws2_32!__WSAFDIsSet fd:0x{:x} set:0x{:x}", fd, set_ptr);
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}
