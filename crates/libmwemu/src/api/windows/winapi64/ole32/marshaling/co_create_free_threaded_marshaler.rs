use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoCreateFreeThreadedMarshaler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let punk_outer = api_try!(emu, call.arg(emu, 0));
    let ppunk_marshal = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("ole32://IMarshal");
    if ppunk_marshal != 0 {
        emu.maps.write_qword(ppunk_marshal, hndl);
    }
    log_red!(emu, "ole32!CoCreateFreeThreadedMarshaler =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
