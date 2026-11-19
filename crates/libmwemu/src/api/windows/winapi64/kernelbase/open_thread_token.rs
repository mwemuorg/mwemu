use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let thread_handle = api_try!(emu, call.arg(emu, 0));
    let desired_access = api_try!(emu, call.arg_u32(emu, 1));
    let open_as_self = api_try!(emu, call.arg_u32(emu, 2));
    let token_handle_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "kernelbase!OpenThreadToken thread: 0x{:x} access: 0x{:x} as_self: {}",
        thread_handle,
        desired_access,
        open_as_self
    );
    if token_handle_ptr != 0 {
        let token = helper::handler_create("token");
        emu.maps.write_qword(token_handle_ptr, token);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
