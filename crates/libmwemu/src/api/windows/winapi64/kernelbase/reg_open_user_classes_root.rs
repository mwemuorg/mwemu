use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegOpenUserClassesRoot(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg(emu, 0));
    let _reserved = api_try!(emu, call.arg(emu, 1));
    let sam = api_try!(emu, call.arg(emu, 2));
    let result_ptr = api_try!(emu, call.arg(emu, 3));
    let hndl = helper::handler_create("regkey_HKCR_user");
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!RegOpenUserClassesRoot token:0x{:x} => 0x{:x}",
        token,
        hndl
    );
    call.set_return(emu, 0); // ERROR_SUCCESS
}
