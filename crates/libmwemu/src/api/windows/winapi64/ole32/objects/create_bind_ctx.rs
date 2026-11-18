use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateBindCtx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reserved = api_try!(emu, call.arg(emu, 0));
    let ppbc = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("ole32://IBindCtx");
    if ppbc != 0 {
        emu.maps.write_qword(ppbc, hndl);
    }
    log_red!(emu, "ole32!CreateBindCtx =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
