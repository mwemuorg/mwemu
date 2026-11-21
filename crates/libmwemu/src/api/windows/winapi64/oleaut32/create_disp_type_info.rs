use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDispTypeInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pidata = api_try!(emu, call.arg(emu, 0));
    let lcid = api_try!(emu, call.arg(emu, 1));
    let pptinfo = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("oleaut32://ITypeInfo");
    if pptinfo != 0 {
        emu.maps.write_qword(pptinfo, hndl);
    }
    log_red!(emu, "oleaut32!CreateDispTypeInfo =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}
