use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoGetMalloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _context = api_try!(emu, call.arg(emu, 0));
    let ppv = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("com_malloc");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(emu, "ole32!CoGetMalloc => 0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
