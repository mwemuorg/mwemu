use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDataCache(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let punk_outer = api_try!(emu, call.arg(emu, 0));
    let rclsid = api_try!(emu, call.arg(emu, 1));
    let riid = api_try!(emu, call.arg(emu, 2));
    let ppv = api_try!(emu, call.arg(emu, 3));
    let hndl = helper::handler_create("ole32://IOleCache");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(
        emu,
        "ole32!CreateDataCache outer:0x{:x} clsid:0x{:x} =0x{:x}",
        punk_outer,
        rclsid,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
