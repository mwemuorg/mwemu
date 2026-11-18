use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegOpenCurrentUser(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sam_desired = api_try!(emu, call.arg(emu, 0));
    let result_ptr = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("regkey_HKCU");
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!RegOpenCurrentUser sam:0x{:x} => 0x{:x}",
        sam_desired,
        hndl
    );
    call.set_return(emu, 0); // ERROR_SUCCESS
}
