use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let process_handle = api_try!(emu, call.arg_u32(emu, 0));
    let desired_access = api_try!(emu, call.arg_u32(emu, 1));
    let token_handle_ptr = api_try!(emu, call.arg(emu, 2));
    let token = helper::handler_create("token");
    if token_handle_ptr != 0 {
        emu.maps.write_dword(token_handle_ptr, token as u32);
    }
    log_red!(
        emu,
        "advapi32!OpenProcessToken proc:0x{:x} access:0x{:x} => 0x{:x}",
        process_handle,
        desired_access,
        token
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}
