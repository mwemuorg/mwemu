use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DuplicateToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let existing_token = api_try!(emu, call.arg(emu, 0));
    let imp_level = api_try!(emu, call.arg_u32(emu, 1));
    let new_token_ptr = api_try!(emu, call.arg(emu, 2));
    let new_hndl = helper::handler_create("dup_token");
    if new_token_ptr != 0 {
        emu.maps.write_dword(new_token_ptr, new_hndl as u32);
    }
    log_red!(
        emu,
        "advapi32!DuplicateToken src:0x{:x} level:{} => 0x{:x}",
        existing_token,
        imp_level,
        new_hndl
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}
