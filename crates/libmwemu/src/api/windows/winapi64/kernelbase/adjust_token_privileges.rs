use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg(emu, 0));
    let disable_all = api_try!(emu, call.arg(emu, 1));
    let new_state = api_try!(emu, call.arg(emu, 2));
    let buf_len = api_try!(emu, call.arg(emu, 3));
    let prev_state = api_try!(emu, call.arg(emu, 4));
    let ret_len = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "kernelbase!AdjustTokenPrivileges token:0x{:x} disable_all:{} new:0x{:x}",
        token,
        disable_all,
        new_state
    );
    if ret_len != 0 {
        emu.maps.write_dword(ret_len, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
