use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCPInfoExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cp = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    let info_ptr = api_try!(emu, call.arg(emu, 2));
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 1);
        emu.maps.write_byte(info_ptr + 4, 0x3f);
    }
    log_red!(emu, "kernelbase!GetCPInfoExW cp:{} flags:0x{:x}", cp, flags);
    call.set_return(emu, 1);
}
