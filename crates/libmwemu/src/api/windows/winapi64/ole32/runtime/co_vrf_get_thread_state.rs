use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CoVrfGetThreadState(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let state_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoVrfGetThreadState state_ptr:0x{:x}", state_ptr);
    if state_ptr != 0 {
        emu.maps.write_dword(state_ptr, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
