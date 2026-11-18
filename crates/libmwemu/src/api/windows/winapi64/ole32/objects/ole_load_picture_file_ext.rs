use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OleLoadPictureFileExt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "ole32!OleLoadPictureFileExt arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let out_ptr = api_try!(emu, call.arg(emu, 2));
    let handle = helper::handler_create("OleLoadPictureFileExt");
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
    for _ in 0..1 {}
}
