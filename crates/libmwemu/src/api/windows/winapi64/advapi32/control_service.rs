use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ControlService(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hsvc = api_try!(emu, call.arg(emu, 0));
    let control = api_try!(emu, call.arg(emu, 1));
    let status_ptr = api_try!(emu, call.arg(emu, 2));
    if status_ptr != 0 {
        emu.maps.write_dword(status_ptr, 0x10);
        emu.maps.write_dword(status_ptr + 4, 1);
    }
    log_red!(
        emu,
        "advapi32!ControlService svc:0x{:x} ctrl:{}",
        hsvc,
        control
    );
    call.set_return(emu, 1);
}
