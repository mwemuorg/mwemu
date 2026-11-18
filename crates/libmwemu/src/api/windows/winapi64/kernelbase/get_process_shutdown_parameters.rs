use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessShutdownParameters(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let level_ptr = api_try!(emu, call.arg(emu, 0));
    let flags_ptr = api_try!(emu, call.arg(emu, 1));
    if level_ptr != 0 {
        emu.maps.write_dword(level_ptr, 0x280); // default = 640
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    log_red!(emu, "kernelbase!GetProcessShutdownParameters => 0x280");
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
