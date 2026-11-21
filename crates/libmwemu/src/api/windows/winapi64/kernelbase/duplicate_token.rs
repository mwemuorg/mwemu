use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DuplicateToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg(emu, 0));
    let imp_level = api_try!(emu, call.arg(emu, 1));
    let out_ptr = api_try!(emu, call.arg(emu, 2));
    let new_token = helper::handler_create("token");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, new_token);
    }
    log_red!(
        emu,
        "kernelbase!DuplicateToken h:0x{:x} level:{} => 0x{:x}",
        token,
        imp_level,
        new_token
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
