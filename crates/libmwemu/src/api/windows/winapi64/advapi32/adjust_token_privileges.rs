use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token_handle = api_try!(emu, call.arg(emu, 0));
    let disable_all = api_try!(emu, call.arg(emu, 1));
    let new_state = api_try!(emu, call.arg(emu, 2));
    let buffer_length = api_try!(emu, call.arg_u32(emu, 3));
    let previous_state = api_try!(emu, call.arg(emu, 4));
    let return_length = api_try!(emu, call.arg(emu, 5));

    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges handle=0x{:x} disableAll={}",
        token_handle,
        disable_all
    );

    call.set_return(emu, 1);
}
