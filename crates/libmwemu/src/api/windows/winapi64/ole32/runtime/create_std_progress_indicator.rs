use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateStdProgressIndicator(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd_parent = api_try!(emu, call.arg(emu, 0));
    let psz_title = api_try!(emu, call.arg(emu, 1));
    let pp_ibsc = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("ole32://IBindStatusCallback");
    if pp_ibsc != 0 {
        emu.maps.write_qword(pp_ibsc, hndl);
    }
    log_red!(emu, "ole32!CreateStdProgressIndicator =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
