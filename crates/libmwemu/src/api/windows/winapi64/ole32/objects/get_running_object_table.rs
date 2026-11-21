use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetRunningObjectTable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reserved = api_try!(emu, call.arg(emu, 0));
    let pp_rot = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("ole32://IRunningObjectTable");
    if pp_rot != 0 {
        emu.maps.write_qword(pp_rot, hndl);
    }
    log_red!(emu, "ole32!GetRunningObjectTable =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
