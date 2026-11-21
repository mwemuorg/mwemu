use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCPInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cp = api_try!(emu, call.arg(emu, 0));
    let info_ptr = api_try!(emu, call.arg(emu, 1));
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 1); // MaxCharSize
        emu.maps.write_byte(info_ptr + 4, 0x3f); // DefaultChar[0] = '?'
    }
    log_red!(emu, "kernelbase!GetCPInfo cp:{}", cp);
    call.set_return(emu, 1);
}
