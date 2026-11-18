use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DuplicateTokenEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg(emu, 0));
    let desired_access = api_try!(emu, call.arg(emu, 1));
    let _security_attr = api_try!(emu, call.arg(emu, 2));
    let imp_level = api_try!(emu, call.arg(emu, 3));
    let token_type = api_try!(emu, call.arg(emu, 4));
    let out_ptr = api_try!(emu, call.arg(emu, 5));
    let new_token = helper::handler_create("token");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, new_token);
    }
    log_red!(
        emu,
        "kernelbase!DuplicateTokenEx h:0x{:x} access:0x{:x} level:{} type:{} => 0x{:x}",
        token,
        desired_access,
        imp_level,
        token_type,
        new_token
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
