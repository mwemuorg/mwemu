use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DuplicateTokenEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let existing_token = api_try!(emu, call.arg(emu, 0));
    let desired_access = api_try!(emu, call.arg_u32(emu, 1));
    let _lp_attrs = api_try!(emu, call.arg_u32(emu, 2));
    let imp_level = api_try!(emu, call.arg_u32(emu, 3));
    let token_type = api_try!(emu, call.arg_u32(emu, 4));
    let new_token_ptr = api_try!(emu, call.arg(emu, 5));
    let new_hndl = helper::handler_create("dup_token_ex");
    if new_token_ptr != 0 {
        emu.maps.write_dword(new_token_ptr, new_hndl as u32);
    }
    log_red!(
        emu,
        "advapi32!DuplicateTokenEx src:0x{:x} access:0x{:x} level:{} type:{} => 0x{:x}",
        existing_token,
        desired_access,
        imp_level,
        token_type,
        new_hndl
    );
    call.cleanup_args(emu, 6);
    call.set_return(emu, 1);
}
