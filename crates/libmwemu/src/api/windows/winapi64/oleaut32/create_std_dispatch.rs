use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateStdDispatch(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let punk_outer = api_try!(emu, call.arg(emu, 0));
    let pv_this = api_try!(emu, call.arg(emu, 1));
    let ptinfo = api_try!(emu, call.arg(emu, 2));
    let ppunk_std_disp = api_try!(emu, call.arg(emu, 3));
    let hndl = helper::handler_create("oleaut32://IDispatch");
    if ppunk_std_disp != 0 {
        emu.maps.write_qword(ppunk_std_disp, hndl);
    }
    log_red!(emu, "oleaut32!CreateStdDispatch =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}
