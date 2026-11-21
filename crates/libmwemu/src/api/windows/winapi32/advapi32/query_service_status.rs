use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn QueryServiceStatus(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hsvc = api_try!(emu, call.arg_u32(emu, 0));
    let status_ptr = api_try!(emu, call.arg(emu, 1));
    if status_ptr != 0 {
        emu.maps.write_dword(status_ptr, 0x10);
        emu.maps.write_dword(status_ptr + 4, 4);
        emu.maps.write_dword(status_ptr + 8, 0x1f);
        emu.maps.write_dword(status_ptr + 12, 0);
        emu.maps.write_dword(status_ptr + 16, 0);
        emu.maps.write_dword(status_ptr + 20, 0);
        emu.maps.write_dword(status_ptr + 24, 0);
    }
    log_red!(emu, "advapi32!QueryServiceStatus svc:0x{:x}", hsvc);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}
