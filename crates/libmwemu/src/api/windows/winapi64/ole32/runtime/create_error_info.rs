use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateErrorInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pp_errinfo = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("ole32://ICreateErrorInfo");
    if pp_errinfo != 0 {
        emu.maps.write_qword(pp_errinfo, hndl);
    }
    log_red!(emu, "ole32!CreateErrorInfo =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
