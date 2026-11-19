use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CoFileTimeNow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoFileTimeNow arg1:0x{:x}", arg1);
    if api_try!(emu, call.arg(emu, 0)) != 0 {
        emu.maps.write_qword(api_try!(emu, call.arg(emu, 0)), 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
