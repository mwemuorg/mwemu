use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadTypeLibEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sz_file = api_try!(emu, call.arg(emu, 0));
    let reg_kind = api_try!(emu, call.arg(emu, 1));
    let pptlib = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("oleaut32://ITypeLib");
    if pptlib != 0 {
        emu.maps.write_qword(pptlib, hndl);
    }
    let path = emu.maps.read_wide_string(sz_file);
    log_red!(emu, "oleaut32!LoadTypeLibEx '{}' =0x{:x}", path, hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}
