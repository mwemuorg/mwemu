use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetStartupInfoW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let info_ptr = api_try!(emu, call.arg(emu, 0));
    if info_ptr != 0 {
        let cb = if true { 104u32 } else { 68u32 };
        emu.maps.write_dword(info_ptr, cb);
    }
    log_red!(emu, "kernelbase!GetStartupInfoW");
}
