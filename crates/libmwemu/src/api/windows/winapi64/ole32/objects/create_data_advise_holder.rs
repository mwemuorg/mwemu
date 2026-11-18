use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDataAdviseHolder(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pp_dah = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("ole32://IDataAdviseHolder");
    if pp_dah != 0 {
        emu.maps.write_qword(pp_dah, hndl);
    }
    log_red!(emu, "ole32!CreateDataAdviseHolder =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
