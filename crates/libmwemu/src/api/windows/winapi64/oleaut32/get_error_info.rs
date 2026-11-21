use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetErrorInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reserved = api_try!(emu, call.arg(emu, 0));
    let pperrinfo = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "oleaut32!GetErrorInfo reserved: {} pperrinfo: 0x{:x}",
        reserved,
        pperrinfo
    );
    if pperrinfo != 0 {
        emu.maps.write_qword(pperrinfo, 0); // no error info
    }
    emu.set_last_error(0);
    call.set_return(emu, 1); // S_FALSE (no error info available)
}
