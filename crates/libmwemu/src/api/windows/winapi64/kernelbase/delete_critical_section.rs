use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DeleteCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cs_ptr = api_try!(emu, call.arg(emu, 0));
    if cs_ptr != 0 {
        emu.maps.write_qword(cs_ptr, 0);
    }
    log_red!(emu, "kernelbase!DeleteCriticalSection 0x{:x}", cs_ptr);
}
