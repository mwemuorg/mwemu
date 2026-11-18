use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateILockBytesOnHGlobal(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hglobal = api_try!(emu, call.arg(emu, 0));
    let f_delete = api_try!(emu, call.arg(emu, 1));
    let pplkbyt = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("ole32://ILockBytes");
    if pplkbyt != 0 {
        emu.maps.write_qword(pplkbyt, hndl);
    }
    log_red!(emu, "ole32!CreateILockBytesOnHGlobal =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
