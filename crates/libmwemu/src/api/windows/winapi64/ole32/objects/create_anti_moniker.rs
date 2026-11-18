use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateAntiMoniker(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ppmk = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk != 0 {
        emu.maps.write_qword(ppmk, hndl);
    }
    log_red!(
        emu,
        "ole32!CreateAntiMoniker ppmk:0x{:x} =0x{:x}",
        ppmk,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
