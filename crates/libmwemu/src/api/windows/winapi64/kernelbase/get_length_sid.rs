use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetLengthSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!GetLengthSid");
    if api_try!(emu, call.arg(emu, 0)) != 0 && emu.maps.is_mapped(api_try!(emu, call.arg(emu, 0))) {
        let _ = emu
            .maps
            .read_dword(api_try!(emu, call.arg(emu, 0)))
            .unwrap_or(0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 28);
}
