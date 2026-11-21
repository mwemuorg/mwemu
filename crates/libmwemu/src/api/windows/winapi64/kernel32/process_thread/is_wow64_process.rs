use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsWow64Process(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _process = api_try!(emu, call.arg(emu, 0));
    let out_ptr = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "kernel32!IsWow64Process out: 0x{:x}", out_ptr);

    emu.maps.write_qword(out_ptr, 0); // FALSE — 64-bit process
    call.set_return(emu, 1);
}
