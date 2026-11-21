use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg_u32(emu, 0));
    let disable_all = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges token=0x{:x} disableAll={}",
        token,
        disable_all
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
