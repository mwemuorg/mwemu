use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateFileMoniker(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpsz_path_name = api_try!(emu, call.arg(emu, 0));
    let ppmk = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk != 0 {
        emu.maps.write_qword(ppmk, hndl);
    }
    let path = emu.maps.read_wide_string(lpsz_path_name);
    log_red!(emu, "ole32!CreateFileMoniker path:'{}' =0x{:x}", path, hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
