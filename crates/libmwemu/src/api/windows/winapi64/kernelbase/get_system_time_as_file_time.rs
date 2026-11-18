use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ft_ptr = api_try!(emu, call.arg(emu, 0));
    if ft_ptr != 0 {
        emu.maps.write_dword(ft_ptr, 0xd53e8000);
        emu.maps.write_dword(ft_ptr + 4, 0x019db1de);
    }
    log_red!(emu, "kernelbase!GetSystemTimeAsFileTime");
}
