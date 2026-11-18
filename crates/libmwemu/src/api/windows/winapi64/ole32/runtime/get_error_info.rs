use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetErrorInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _reserved = api_try!(emu, call.arg(emu, 0));
    let pperrinfo = api_try!(emu, call.arg(emu, 1));
    if pperrinfo != 0 {
        emu.maps.write_dword(pperrinfo, 0); // no error info
    }
    log_red!(emu, "ole32!GetErrorInfo => S_FALSE (no error info)");
    call.set_return(emu, 1); // S_FALSE
}
