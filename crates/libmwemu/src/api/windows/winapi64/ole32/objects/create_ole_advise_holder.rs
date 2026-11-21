use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateOleAdviseHolder(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pp_oah = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("ole32://IOleAdviseHolder");
    if pp_oah != 0 {
        emu.maps.write_qword(pp_oah, hndl);
    }
    log_red!(emu, "ole32!CreateOleAdviseHolder =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
