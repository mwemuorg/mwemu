use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegOpenCurrentUser(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _sam_desired = api_try!(emu, call.arg(emu, 0));
    let phk_result = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("registry://HKEY_CURRENT_USER");
    if phk_result != 0 {
        emu.maps.write_qword(phk_result, hndl);
    }
    log_red!(emu, "advapi32!RegOpenCurrentUser => 0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
