use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn getprotobyname(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!getprotobyname name:'{}'", name);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}
