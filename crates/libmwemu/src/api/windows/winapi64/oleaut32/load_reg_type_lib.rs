use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadRegTypeLib(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rguid = api_try!(emu, call.arg(emu, 0));
    let w_ver_major = api_try!(emu, call.arg_u16(emu, 1));
    let w_ver_minor = api_try!(emu, call.arg_u16(emu, 2));
    let lcid = api_try!(emu, call.arg_u32(emu, 3));
    let pptlib = api_try!(emu, call.arg(emu, 4));
    let hndl = helper::handler_create("oleaut32://ITypeLib");
    if pptlib != 0 {
        emu.maps.write_qword(pptlib, hndl);
    }
    log_red!(emu, "oleaut32!LoadRegTypeLib =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}
