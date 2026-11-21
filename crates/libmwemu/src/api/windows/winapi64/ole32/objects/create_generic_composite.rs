use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateGenericComposite(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pmk_first = api_try!(emu, call.arg(emu, 0));
    let pmk_rest = api_try!(emu, call.arg(emu, 1));
    let ppmk_composite = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk_composite != 0 {
        emu.maps.write_qword(ppmk_composite, hndl);
    }
    log_red!(emu, "ole32!CreateGenericComposite =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
