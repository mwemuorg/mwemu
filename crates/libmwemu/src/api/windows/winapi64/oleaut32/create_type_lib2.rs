use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateTypeLib2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sys_kind = api_try!(emu, call.arg(emu, 0));
    let sz_file = api_try!(emu, call.arg(emu, 1));
    let pp_ctlib = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create("oleaut32://ICreateTypeLib2");
    if pp_ctlib != 0 {
        emu.maps.write_qword(pp_ctlib, hndl);
    }
    log_red!(emu, "oleaut32!CreateTypeLib2 =0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}
