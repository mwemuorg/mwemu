use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let thread_handle = api_try!(emu, call.arg_u32(emu, 0));
    let desired_access = api_try!(emu, call.arg_u32(emu, 1));
    let open_as_self = api_try!(emu, call.arg_u32(emu, 2)) != 0;
    let token_handle_ptr = api_try!(emu, call.arg(emu, 3));
    let token = helper::handler_create("token");
    if token_handle_ptr != 0 {
        emu.maps.write_dword(token_handle_ptr, token as u32);
    }
    log_red!(
        emu,
        "advapi32!OpenThreadToken thread:0x{:x} access:0x{:x} => 0x{:x}",
        thread_handle,
        desired_access,
        token
    );
    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}
