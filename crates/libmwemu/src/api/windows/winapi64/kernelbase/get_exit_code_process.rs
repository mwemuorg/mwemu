use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetExitCodeProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let exit_code_ptr = api_try!(emu, call.arg(emu, 1));
    if exit_code_ptr != 0 {
        emu.maps.write_dword(exit_code_ptr, 259); // STILL_ACTIVE
    }
    log_red!(emu, "kernelbase!GetExitCodeProcess handle:0x{:x}", handle);
    call.set_return(emu, 1);
}
