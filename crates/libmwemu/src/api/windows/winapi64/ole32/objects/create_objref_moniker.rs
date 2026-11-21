use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateObjrefMoniker(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let punk = api_try!(emu, call.arg(emu, 0));
    let ppmk = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk != 0 {
        emu.maps.write_qword(ppmk, hndl);
    }
    log_red!(emu, "ole32!CreateObjrefMoniker =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
