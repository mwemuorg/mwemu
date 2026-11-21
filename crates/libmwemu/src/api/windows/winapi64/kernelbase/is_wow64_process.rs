use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsWow64Process(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let process = api_try!(emu, call.arg(emu, 0));
    let wow64_ptr = api_try!(emu, call.arg(emu, 1));
    if wow64_ptr != 0 {
        emu.maps.write_dword(wow64_ptr, 0);
    }
    log_red!(
        emu,
        "kernelbase!IsWow64Process handle:0x{:x} => FALSE",
        process
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
