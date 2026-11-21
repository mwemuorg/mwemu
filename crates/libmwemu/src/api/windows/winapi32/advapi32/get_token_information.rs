use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token_handle = api_try!(emu, call.arg_u32(emu, 0));
    let info_class = api_try!(emu, call.arg_u32(emu, 1));
    let info_buf = api_try!(emu, call.arg(emu, 2));
    let buf_len = api_try!(emu, call.arg_u32(emu, 3));
    let ret_len_ptr = api_try!(emu, call.arg(emu, 4));
    if ret_len_ptr != 0 {
        emu.maps.write_dword(ret_len_ptr, 4);
    }
    if info_buf != 0 && buf_len >= 4 {
        emu.maps.write_dword(info_buf, 1);
    }
    log_red!(
        emu,
        "advapi32!GetTokenInformation token:0x{:x} class:{}",
        token_handle,
        info_class
    );
    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}
