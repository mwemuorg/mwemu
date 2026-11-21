use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCPInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let codepage = api_try!(emu, call.arg_u32(emu, 0));
    let info_ptr = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!GetCPInfo {} 0x{}", codepage, info_ptr);

    // TODO: put something in lp_cp_info?

    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);

    // https://learn.microsoft.com/en-us/windows/win32/api/winnls/ns-winnls-cpinfo
}
