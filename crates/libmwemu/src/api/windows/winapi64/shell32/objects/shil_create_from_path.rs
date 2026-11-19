use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHILCreateFromPath(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let psz_path = api_try!(emu, call.arg(emu, 0));
    let ppidl = api_try!(emu, call.arg(emu, 1));
    let rgf_in_out = api_try!(emu, call.arg(emu, 2));
    let path = emu.maps.read_wide_string(psz_path);
    let hndl = helper::handler_create(&format!("pidl://{}", path));
    if ppidl != 0 {
        emu.maps.write_qword(ppidl, hndl);
    }
    log_red!(emu, "shell32!SHILCreateFromPath '{}' =0x{:x}", path, hndl);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
