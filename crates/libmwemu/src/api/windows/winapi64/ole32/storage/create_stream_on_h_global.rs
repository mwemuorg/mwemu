use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateStreamOnHGlobal(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hglobal = api_try!(emu, call.arg(emu, 0));
    let f_delete = api_try!(emu, call.arg(emu, 1));
    let ppstm = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("ole32://IStream");
    if ppstm != 0 {
        emu.maps.write_qword(ppstm, hndl);
    }
    log_red!(emu, "ole32!CreateStreamOnHGlobal =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
