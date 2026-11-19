use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHGetRealIDL(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let psf = api_try!(emu, call.arg(emu, 0));
    let pidl_simple = api_try!(emu, call.arg(emu, 1));
    let ppidl_real = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("pidl://real");
    if ppidl_real != 0 {
        emu.maps.write_qword(ppidl_real, hndl);
    }
    log_red!(emu, "shell32!SHGetRealIDL =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
