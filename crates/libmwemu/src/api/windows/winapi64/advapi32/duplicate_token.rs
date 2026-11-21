use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DuplicateToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let existing_token = api_try!(emu, call.arg(emu, 0));
    let _impersonation_level = api_try!(emu, call.arg(emu, 1));
    let dup_token_ptr = api_try!(emu, call.arg(emu, 2));
    let new_token = helper::handler_create("dup_token");
    if dup_token_ptr != 0 {
        emu.maps.write_qword(dup_token_ptr, new_token);
    }
    log_red!(
        emu,
        "advapi32!DuplicateToken src:0x{:x} => 0x{:x}",
        existing_token,
        new_token
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
